//! <div align="center">
//!   <a href="https://packrinth.thijzert.nl"><img src="https://github.com/Thijzert123/packrinth/blob/ff8455254b966d7879ca2c378a4350c1a56cbfc6/logo.png?raw=true" alt="logo" width=100 height=100 /></a>
//!
//!   <h1>Packrinth</h1>
//!   CLI tool for creating and managing Minecraft modpacks with Modrinth projects
//!
//!   <p></p>
//!
//!   [![Crates.io Version](https://img.shields.io/crates/v/packrinth?style=for-the-badge)](https://crates.io/crates/packrinth)
//!   [![Crates.io Total Downloads](https://img.shields.io/crates/d/packrinth?style=for-the-badge)](https://crates.io/crates/packrinth)
//! </div>
//!
//! ---
//!
//! This library provides utilities for integrating with Packrinth. For example,
//! the module `config` gives structs for reading and editing Packrinth configuration files.
//! The module `modrinth` can be used to retrieve data from Modrinth and convert it to
//! Packrinth-compatible structs.
//!
//! If you just want to use the Packrinth CLI, go to <https://packrinth.thijzert.nl>
//! to see how to use it.

#![warn(clippy::pedantic)]

use std::io::Write;
pub mod config;
pub mod modrinth;

use crate::config::{BranchConfig, BranchFiles, BranchFilesProject, Modpack, ProjectSettings};
use crate::modrinth::{Env, File, FileResult, SideSupport, VersionDependency};
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest_retry::RetryTransientMiddleware;
use reqwest_retry::policies::ExponentialBackoff;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::fs::OpenOptions;
use std::path::Path;
use std::sync::OnceLock;
use std::time::Duration;
use std::{fs, io};
use zip::ZipArchive;
use zip::result::ZipResult;

/// The name of the target directory
pub const TARGET_DIRECTORY: &str = "target";

static CLIENT: OnceLock<ClientWithMiddleware> = OnceLock::new();
const USER_AGENT: &str = concat!(
    "Thijzert123",
    "/",
    "packrinth",
    "/",
    env!("CARGO_PKG_VERSION")
);

fn request_text<T: ToString + ?Sized>(full_url: &T) -> Result<String, PackrinthError> {
    let client = CLIENT.get_or_init(|| {
        let retry_policy = ExponentialBackoff::builder()
            .build_with_total_retry_duration(Duration::from_secs(60 * 2));
        ClientBuilder::new(
            reqwest::Client::builder()
                .user_agent(USER_AGENT)
                .build()
                .expect("Failed to build request client"),
        )
        .with(RetryTransientMiddleware::new_with_policy(retry_policy))
        .build()
    });

    let runtime = tokio::runtime::Runtime::new().expect("Failed to create runtime");
    let response = runtime
        .block_on(client.get(full_url.to_string()).send())
        .expect("Failed to get response");
    match runtime.block_on(response.text()) {
        Ok(text) => Ok(text),
        Err(error) => Err(PackrinthError::RequestFailed {
            url: full_url.to_string(),
            error_message: error.to_string(),
        }),
    }
}

pub const MRPACK_CONFIG_FILE_NAME: &str = "modrinth.index.json";

// TODO api docs
// Allow because these bools aren't here because this struct is a state machine.
// All bool value combinations are valid, so no worries at all, Clippy!
#[allow(clippy::struct_excessive_bools)]
pub struct ProjectUpdater<'a> {
    pub branch_name: &'a str,
    pub branch_config: &'a BranchConfig,
    pub branch_files: &'a mut BranchFiles,
    pub slug_project_id: &'a str,
    pub project_settings: &'a ProjectSettings,
    pub require_all: bool,
    pub no_beta: bool,
    pub no_alpha: bool,
}

// TODO api docs
pub enum ProjectUpdateResult {
    Added(Vec<VersionDependency>),
    Skipped,
    NotFound,
    Failed(PackrinthError),
}

impl ProjectUpdater<'_> {
    // TODO api docs
    pub fn update_project(&mut self) -> ProjectUpdateResult {
        match File::from_project(
            &self.branch_name.to_string(),
            self.branch_config,
            self.slug_project_id,
            self.project_settings,
            self.no_beta,
            self.no_alpha,
        ) {
            FileResult::Ok {
                mut file,
                dependencies,
            } => {
                self.branch_files.projects.push(BranchFilesProject {
                    name: file.project_name.clone(),
                    id: Some(self.slug_project_id.to_string()),
                });

                if self.require_all {
                    file.env = Some(Env {
                        client: SideSupport::Required,
                        server: SideSupport::Required,
                    });
                }

                self.branch_files.files.push(file);
                ProjectUpdateResult::Added(dependencies)
            }
            FileResult::Skipped => ProjectUpdateResult::Skipped,
            FileResult::NotFound => ProjectUpdateResult::NotFound,
            FileResult::Err(error) => ProjectUpdateResult::Failed(error),
        }
    }
}

#[derive(Debug)]
pub struct ProjectMarkdownTable<'a> {
    pub column_names: Vec<&'a str>,
    pub project_map: HashMap<BranchFilesProject, HashMap<String, Option<()>>>,
}

impl Display for ProjectMarkdownTable<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // Write column names
        writeln!(f, "|{}|", self.column_names.join("|"))?;

        // Write alignment text (:-- is left, :-: is center)
        write!(f, "|:--|")?;
        // Use 1..len because column names include the 'Name' for the project column
        for _ in 1..self.column_names.len() {
            write!(f, ":-:|")?;
        }
        writeln!(f)?;

        let mut sorted_project_map: Vec<_> = self.project_map.iter().collect();
        // Sort by key (human name of project)
        sorted_project_map.sort_by(|a, b| a.0.name.cmp(&b.0.name));

        let mut iter = sorted_project_map.iter().peekable();
        while let Some(project) = iter.next() {
            if let Some(id) = &project.0.id {
                // If project has an id (not a manual file), write a Markdown link.
                let mut project_url = "https://modrinth.com/project/".to_string();
                project_url.push_str(id);
                write!(f, "|[{}]({})|", project.0.name, project_url)?;
            } else {
                write!(f, "|{}|", project.0.name)?;
            }

            let mut sorted_branch_map: Vec<_> = project.1.iter().collect();
            // Sort by key (human name of project)
            sorted_branch_map.sort_by(|a, b| a.0.cmp(b.0));

            for branch in sorted_branch_map {
                let icon = match branch.1 {
                    Some(()) => "✅",
                    None => "❌",
                };
                write!(f, "{icon}|")?;
            }

            // Print newline except for the last time of this loop.
            if iter.peek().is_some() {
                writeln!(f)?;
            }
        }

        Ok(())
    }
}

// TODO extend modrinth api structs to have all possible values, not just the ones required by packrinth

// TODO api doc
pub struct GitUtils;

impl GitUtils {
    // TODO api doc
    pub fn initialize_modpack_repo(directory: &Path) -> Result<(), PackrinthError> {
        if let Err(error) = gix::init(directory) {
            // If the repo already exists, don't show an error.
            if !matches!(
                &error,
                gix::init::Error::Init(gix::create::Error::DirectoryExists { path })
                    if path.file_name() == Some(std::ffi::OsStr::new(".git"))
            ) {
                return Err(PackrinthError::FailedToInitGitRepoWhileInitModpack {
                    error_message: error.to_string(),
                });
            }
        }

        let gitignore_path = directory.join(".gitignore");
        if let Ok(exists) = fs::exists(&gitignore_path)
            && !exists
            && let Ok(gitignore_file) = OpenOptions::new()
                .append(true)
                .create(true)
                .open(gitignore_path)
        {
            // If the gitignore file can't be written to, so be it.
            let _ = writeln!(&gitignore_file, "# Exported files");
            let _ = writeln!(&gitignore_file, "{TARGET_DIRECTORY}");
            let _ = gitignore_file.sync_all();
        }

        Ok(())
    }

    /// Checks if the modpack is dirty.
    ///
    /// It does this by checking whether the directory of the modpack
    /// has uncommitted changes. If any errors occur (for example, if no Git repository exists),
    /// `false` will be returned.
    #[must_use]
    pub fn modpack_is_dirty(modpack: &Modpack) -> bool {
        let git_repo = match gix::open(&modpack.directory) {
            Ok(git_repo) => git_repo,
            Err(_error) => return false,
        };

        git_repo.is_dirty().unwrap_or(false)
    }
}

/// Struct representative of all versions of a crate on the `crates.io` API.
#[derive(Debug, Serialize, Deserialize)]
pub struct CratesIoVersions {
    pub versions: Vec<CratesIoVersion>,
}

/// Struct representative of a version on the `crates.io` API.
#[derive(Debug, Serialize, Deserialize)]
pub struct CratesIoVersion {
    /// The version number of the crate version.
    pub num: String,
}

impl CratesIoVersions {
    /// Gets `crates.io` versions from a crate name.
    ///
    /// # Errors
    /// - [`PackrinthError::FailedToParseCratesIoResponseJson`] if the response was invalid
    pub fn from_crate(crate_name: &str) -> Result<Self, PackrinthError> {
        let endpoint = format!("/crates/{crate_name}/versions");
        let full_url = format!("https://crates.io/api/v1/{endpoint}");
        let crates_io_response = request_text(&full_url)?;

        match serde_json::from_str::<Self>(&crates_io_response) {
            Ok(versions) => Ok(versions),
            Err(error) => Err(PackrinthError::FailedToParseCratesIoResponseJson {
                crates_io_endpoint: endpoint.to_string(),
                error_message: error.to_string(),
            }),
        }
    }
}

/// Checks if a new Packrinth version is available by checking if a newer semantic version is
/// present on `crates.io`.
///
/// # Errors
/// - [`PackrinthError::FailedToParseSemverVersion`] if parsing a version to a semver version failed
pub fn is_new_version_available() -> Result<Option<String>, PackrinthError> {
    let newest_version = &CratesIoVersions::from_crate(env!("CARGO_PKG_NAME"))?.versions[0].num;
    let newest_version = match semver::Version::parse(newest_version) {
        Ok(version) => version,
        Err(error) => {
            return Err(PackrinthError::FailedToParseSemverVersion {
                version_to_parse: newest_version.clone(),
                error_message: error.to_string(),
            });
        }
    };
    let current_version = env!("CARGO_PKG_VERSION");
    let current_version = match semver::Version::parse(current_version) {
        Ok(version) => version,
        Err(error) => {
            return Err(PackrinthError::FailedToParseSemverVersion {
                version_to_parse: current_version.to_string(),
                error_message: error.to_string(),
            });
        }
    };

    if newest_version > current_version {
        Ok(Some(newest_version.to_string()))
    } else {
        Ok(None)
    }
}

/// An error that can occur while performing Packrinth operations.
#[non_exhaustive]
#[derive(Debug, PartialEq)]
pub enum PackrinthError {
    PathIsFile {
        path: String,
    },
    FailedToCreateDir {
        dir_to_create: String,
        error_message: String,
    },
    FailedToReadToString {
        path_to_read: String,
        error_message: String,
    },
    FailedToParseConfigJson {
        config_path: String,
        error_message: String,
    },
    FailedToParseModrinthResponseJson {
        modrinth_endpoint: String,
        error_message: String,
    },
    FailedToSerialize {
        error_message: String,
    },
    ProjectIsNotAdded {
        project: String,
    },
    OverrideDoesNotExist {
        project: String,
        branch: String,
    },
    NoOverridesForProject {
        project: String,
    },
    NoExclusionsForProject {
        project: String,
    },
    NoInclusionsForProject {
        project: String,
    },
    ProjectAlreadyHasExclusions {
        project: String,
    },
    ProjectAlreadyHasInclusions {
        project: String,
    },
    FailedToWriteFile {
        path_to_write_to: String,
        error_message: String,
    },
    FailedToInitializeFileType {
        file_to_create: String,
        error_message: String,
    },
    DirectoryExpected {
        path_that_should_have_been_dir: String,
    },
    FailedToStartZipFile {
        file_to_start: String,
        error_message: String,
    },
    FailedToWriteToZip {
        to_write: String,
        error_message: String,
    },
    FailedToGetWalkDirEntry {
        error_message: String,
    },
    FailedToStripPath {
        path: String,
    },
    FailedToCopyIntoBuffer,
    FailedToAddZipDir {
        zip_dir_path: String,
    },
    FailedToFinishZip,
    BranchDoesNotExist {
        branch: String,
        error_message: String,
    },
    AttemptedToAddOtherModpack,
    NoModrinthFilesFoundForProject {
        project: String,
    },
    RequestFailed {
        url: String,
        error_message: String,
    },
    FailedToGetCurrentDirectory {
        error_message: String,
    },
    InvalidPackFormat {
        used_pack_format: u16,
    },
    NoBranchSpecified,
    NoInclusionsSpecified,
    NoExclusionsSpecified,
    RepoIsDirty,
    FailedToInitGitRepoWhileInitModpack {
        error_message: String,
    },
    ModpackAlreadyExists {
        directory: String,
    },
    MainModLoaderProvidedButNoVersion,
    ModpackHasNoBranchesToUpdate,
    FailedToCreateZipArchive {
        zip_path: String,
        error_message: String,
    },
    InvalidMrPack {
        mrpack_path: String,
        error_message: String,
    },
    FailedToExtractMrPack {
        mrpack_path: String,
        output_directory: String,
        error_message: String,
    },
    BranchAlreadyExists {
        branch: String,
    },
    FailedToRemoveDir {
        dir_to_remove: String,
        error_message: String,
    },
    FailedToParseCratesIoResponseJson {
        crates_io_endpoint: String,
        error_message: String,
    },
    FailedToParseSemverVersion {
        version_to_parse: String,
        error_message: String,
    },
}

impl PackrinthError {
    /// Returns a message and tip for a [`PackrinthError`], in the form of (message, tip).
    /// It uses the relevant data in the enum value.
    #[must_use]
    pub fn message_and_tip(&self) -> (String, String) {
        let file_an_issue: String =
            "file an issue at https://github.com/Thijzert123/packrinth/issues".to_string();
        match self {
            PackrinthError::PathIsFile { path } => (format!("path {path} is a file"), "remove the file or change the target directory".to_string()),
            PackrinthError::FailedToCreateDir{ dir_to_create, error_message } => (format!("failed to create directory {dir_to_create}: {error_message}"), "check if you have sufficient permissions and if the path already exists".to_string()),
            PackrinthError::FailedToReadToString { path_to_read, error_message } => (format!("failed to read file {path_to_read}: {error_message}"), "check if you have sufficient permissions and if the file exists".to_string()),
            PackrinthError::FailedToParseConfigJson { config_path, error_message } => (format!("config file {config_path} is invalid: {error_message}"), "fix it according to JSON standards".to_string()),
            PackrinthError::FailedToParseModrinthResponseJson { modrinth_endpoint, error_message } => (format!("modrinth response from endpoint {modrinth_endpoint} is invalid: {error_message}"), file_an_issue),
            PackrinthError::FailedToParseCratesIoResponseJson { crates_io_endpoint, error_message } => (format!("crates.io response from endpoint {crates_io_endpoint} is invalid: {error_message}"), file_an_issue),
            PackrinthError::FailedToSerialize{ error_message } => (format!("failed to serialize to a JSON: {error_message}"), file_an_issue),
            PackrinthError::ProjectIsNotAdded { project } => (format!("project {project} is not added to this modpack"), "add it with subcommand: project add".to_string()),
            PackrinthError::OverrideDoesNotExist { project, branch } => (format!("{project} does not have an override for branch {branch}"), "add one with subcommand: project override add".to_string()),
            PackrinthError::NoOverridesForProject { project } => (format!("project {project} doesn't have any overrides"), "add one with subcommand: project override add".to_string()),
            PackrinthError::NoExclusionsForProject { project } => (format!("project {project} doesn't have any exclusions"), "add exclusions with subcommand: project exclude add".to_string()),
            PackrinthError::NoInclusionsForProject { project } => (format!("project {project} doesn't have any inclusions"), "add inclusions with subcommand: project include add".to_string()),
            PackrinthError::ProjectAlreadyHasExclusions { project } => (format!("project {project} already has exclusions"), "you can't have both inclusions and exclusions for one project".to_string()),
            PackrinthError::ProjectAlreadyHasInclusions { project } => (format!("project {project} already has inclusions"), "you can't have both inclusions and exclusions for one project".to_string()),
            PackrinthError::FailedToWriteFile { path_to_write_to, error_message } => (format!("failed to write to file {path_to_write_to}: {error_message}"), "check if you have sufficient permissions and if the file exists".to_string()),
            PackrinthError::FailedToInitializeFileType { file_to_create, error_message } => (format!("failed to create file {file_to_create}: {error_message}"), "check if you have sufficient permissions and if the path already exists".to_string()),
            PackrinthError::DirectoryExpected { path_that_should_have_been_dir } => (format!("expected a directory at {path_that_should_have_been_dir}"), "remove the path if possible".to_string()),
            PackrinthError::FailedToStartZipFile { file_to_start, error_message } => (format!("failed to start zip file at {file_to_start}: {error_message}"), file_an_issue),
            PackrinthError::FailedToWriteToZip { to_write, error_message } => (format!("failed to write {to_write} to zip: {error_message}"), file_an_issue),
            PackrinthError::FailedToGetWalkDirEntry { error_message } => (format!("failed to get entry from WalkDir: {error_message}"), file_an_issue),
            PackrinthError::FailedToStripPath { path } => (format!("failed to strip path {path}"), file_an_issue),
            PackrinthError::FailedToCopyIntoBuffer => ("failed to copy data into buffer for zip".to_string(), file_an_issue),
            PackrinthError::FailedToAddZipDir { zip_dir_path } => (format!("failed to add zip directory {zip_dir_path}"), file_an_issue),
            PackrinthError::FailedToFinishZip => ("failed to finish zip".to_string(), file_an_issue),
            PackrinthError::BranchDoesNotExist { branch, error_message } => (format!("branch {branch} doesn't exist: {error_message}"), "add a branch with subcommand: branch add".to_string()),
            PackrinthError::AttemptedToAddOtherModpack => ("one of the projects is another modpack".to_string(), "remove the modpack project with subcommand: project remove <MODPACK_PROJECT>".to_string()),
            PackrinthError::NoModrinthFilesFoundForProject { project } => (format!("no files found for project {project}"), "check if the project id is spelled correctly or try to remove or add project inclusions, exclusions or overrides".to_string()),
            PackrinthError::RequestFailed { url, error_message } => (format!("request to {url} failed: {error_message}"), format!("check your internet connection or {file_an_issue}")),
            PackrinthError::FailedToGetCurrentDirectory { error_message } => (format!("couldn't get the current directory: {error_message}"), "the current directory may not exist or you have insufficient permissions to access the current directory".to_string()),
            PackrinthError::InvalidPackFormat { used_pack_format } => (format!("pack format {used_pack_format} is not supported by this Packrinth version"), format!("please use a configuration with pack format {}", config::CURRENT_PACK_FORMAT)),
            PackrinthError::NoBranchSpecified => ("no branch specified".to_string(), "specify a branch or remove all with the --all flag".to_string()),
            PackrinthError::NoInclusionsSpecified => ("no inclusions specified".to_string(), "specify inclusions or remove all with the --all flag".to_string()),
            PackrinthError::NoExclusionsSpecified => ("no exclusions specified".to_string(), "specify exclusions or remove all with the --all flag".to_string()),
            PackrinthError::RepoIsDirty => ("git repository has uncommitted changes".to_string(), "pass the --allow-dirty flag to force continuing".to_string()),
            PackrinthError::FailedToInitGitRepoWhileInitModpack { error_message } => (format!("failed to initialize Git repository: {error_message}"), "the modpack itself was initialized successfully, so you can try to initialize a Git repository yourself".to_string()),
            PackrinthError::ModpackAlreadyExists { directory } => (format!("a modpack instance already exists in {directory}"), "to force initializing a new repository, pass the --force flag".to_string()),
            PackrinthError::MainModLoaderProvidedButNoVersion => ("a main mod loader was specified for a branch, but no version was provided".to_string(), "add the loader_version to branch.json".to_string()),
            PackrinthError::ModpackHasNoBranchesToUpdate => ("no branches to update".to_string(), "add a branch with subcommand: branch add".to_string()),
            PackrinthError::FailedToCreateZipArchive { zip_path, error_message } => (format!("failed to create zip archive for zip at {zip_path}: {error_message}"), "check if you have sufficient permissions and if the zip file exists".to_string()),
            PackrinthError::InvalidMrPack { mrpack_path, error_message } => (format!("Modrinth pack at {mrpack_path} is invalid: {error_message}"), "make sure you adhere to the specifications (https://support.modrinth.com/en/articles/8802351-modrinth-modpack-format-mrpack)".to_string()),
            PackrinthError::FailedToExtractMrPack { mrpack_path, output_directory, error_message } => (format!("failed to extract Modrinth pack at {mrpack_path} to {output_directory}: {error_message}"), "check if you have sufficient permissions".to_string()),
            PackrinthError::BranchAlreadyExists { branch } => (format!("branch {branch} already exists"), "you can still continue by passing the --force flag".to_string()),
            PackrinthError::FailedToRemoveDir { dir_to_remove, error_message } => (format!("failed to remove directory {dir_to_remove}: {error_message}"), "check if you have sufficient permissions and if the directory exists".to_string()),
            PackrinthError::FailedToParseSemverVersion { version_to_parse, error_message } => (format!("failed to parse semver version {version_to_parse}: {error_message}"), file_an_issue),
        }
    }
}
