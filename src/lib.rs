#![warn(clippy::pedantic)]
#![allow(clippy::missing_errors_doc, clippy::missing_panics_doc)]

pub mod config;
pub mod modrinth;

use crate::config::Modpack;

/// Checks if the modpack is dirty by checking whether the directory of the modpack
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

/// An error that can occur while performing Packrinth operations.
#[derive(Debug)]
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
    RepoIsDirtyWhileUpdating,
    FailedToInitGitRepoWhileInitModpack {
        error_message: String,
    },
    ModpackAlreadyExists {
        directory: String,
    },
    MainModLoaderProvidedButNoVersion,
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
            PackrinthError::RepoIsDirtyWhileUpdating => ("git repository has uncommitted changes".to_string(), "pass the --allow-dirty flag to force updating".to_string()),
            PackrinthError::FailedToInitGitRepoWhileInitModpack { error_message } => (format!("failed to initialize Git repository: {error_message}"), "the modpack itself was initialized successfully, so you can try to initialize a Git repository yourself".to_string()),
            PackrinthError::ModpackAlreadyExists { directory } => (format!("a modpack instance already exists in {directory}"), "to force initializing a new repository, pass the --force flag".to_string()),
            PackrinthError::MainModLoaderProvidedButNoVersion => ("a main mod loader was specified for a branch, but no version was provided".to_string(), "add the loader_version to branch.json".to_string()),
        }
    }
}
