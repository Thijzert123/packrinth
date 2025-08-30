#![warn(clippy::pedantic)]
#![allow(clippy::missing_errors_doc, clippy::missing_panics_doc)]

pub mod config;
pub mod modrinth;

use crate::config::Modpack;

#[must_use]
pub fn modpack_is_dirty(modpack: &Modpack) -> bool {
    let git_repo = match gix::open(&modpack.directory) {
        Ok(git_repo) => git_repo,
        Err(_error) => return false,
    };

    git_repo.is_dirty().unwrap_or(false)
}

#[derive(Debug)]
pub enum PackrinthError {
    // TODO add original error message to more errors
    PathIsFile(String),                                // path
    FailedToCreateDir(String),                         // dir to create
    FailedToReadToString(String),                      // path to read
    FailedToParseConfigJson(String, String),           // config path, error message
    FailedToParseModrinthResponseJson(String, String), // modrinth endpoint, error message
    FailedToSerialize,                                 //
    ProjectIsNotAdded(String),                         // project
    OverrideDoesNotExist(String, String),              // project, branch
    NoOverridesForProject(String),                     // project
    NoExclusionsForProject(String),                    // project
    NoInclusionsForProject(String),                    // project
    ProjectAlreadyHasExclusions(String),               // project
    ProjectAlreadyHasInclusions(String),               // project
    FailedToWriteFile(String),                         // path to write to
    FailedToInitializeFileType(String),                // file to create
    DirectoryExpected(String),                         // path that should have been a dir
    FailedToStartZipFile(String),                      // file to start
    FailedToWriteToZip(String),                        // what to write
    FailedToGetWalkDirEntry(String),                   // original error
    FailedToStripPath(String),                         // original path that had to be stripped
    FailedToCopyIntoBuffer,                            //
    FailedToAddZipDir(String),                         // zip path to add (is a dir)
    FailedToFinishZip,                                 //
    BranchDoesNotExist(String),                        // branch
    AttemptedToAddOtherModpack,                        //
    NoModrinthFilesFoundForProject(String),            // project
    RequestFailed(String),                             // url
    FailedToGetCurrentDirectory(String),               // original error
    InvalidPackFormat(u16),                            // used pack format
    NoBranchSpecified,                                 //
    NoInclusionsSpecified,                             //
    NoExclusionsSpecified,                             //
    RepoIsDirtyWhileUpdating,                          //
}

impl PackrinthError {
    #[must_use]
    pub fn message_and_tip(&self) -> (String, String) {
        let file_an_issue: String =
            "file an issue at https://github.com/Thijzert123/packrinth/issues".to_string();
        match self {
            PackrinthError::PathIsFile(path) => (format!("path {path} is a file"), "remove the file or change the target directory".to_string()),
            PackrinthError::FailedToCreateDir(dir_to_create) => (format!("failed to create directory {dir_to_create}"), "check if you have sufficient permissions and if the path already exists".to_string()),
            PackrinthError::FailedToReadToString(path_to_read) => (format!("failed to read file {path_to_read}"), "check if you have sufficient permissions and if the file exists".to_string()),
            PackrinthError::FailedToParseConfigJson(config_path, error_message) => (format!("config file {config_path} is invalid: {error_message}"), "fix it according to JSON standards".to_string()),
            PackrinthError::FailedToParseModrinthResponseJson(modrinth_endpoint, error_message) => (format!("modrinth response from endpoint {modrinth_endpoint} is invalid: {error_message}"), file_an_issue),
            PackrinthError::FailedToSerialize => ("failed to serialize to a JSON".to_string(), file_an_issue),
            PackrinthError::ProjectIsNotAdded(project) => (format!("project {project} is not added to this modpack"), "add it with subcommand: project add".to_string()),
            PackrinthError::OverrideDoesNotExist(project, branch) => (format!("{project} does not have an override for branch {branch}"), "add one with subcommand: project override add".to_string()),
            PackrinthError::NoOverridesForProject(project) => (format!("project {project} doesn't have any overrides"), "add one with subcommand: project override add".to_string()),
            PackrinthError::NoExclusionsForProject(project) => (format!("project {project} doesn't have any exclusions"), "add exclusions with subcommand: project exclude add".to_string()),
            PackrinthError::NoInclusionsForProject(project) => (format!("project {project} doesn't have any inclusions"), "add inclusions with subcommand: project include add".to_string()),
            PackrinthError::ProjectAlreadyHasExclusions(project) => (format!("project {project} already has exclusions"), "you can't have both inclusions and exclusions for one project".to_string()),
            PackrinthError::ProjectAlreadyHasInclusions(project) => (format!("project {project} already has inclusions"), "you can't have both inclusions and exclusions for one project".to_string()),
            PackrinthError::FailedToWriteFile(path_to_write_to) => (format!("failed to write to file {path_to_write_to}"), "check if you have sufficient permissions and if the file exists".to_string()),
            PackrinthError::FailedToInitializeFileType(file_to_create) => (format!("failed to create file {file_to_create}"), "check if you have sufficient permissions and if the path already exists".to_string()),
            PackrinthError::DirectoryExpected(path_should_have_been_dir) => (format!("expected a directory at {path_should_have_been_dir}"), "remove the path if possible".to_string()),
            PackrinthError::FailedToStartZipFile(file_to_start) => (format!("failed to start zip file at {file_to_start}"), file_an_issue),
            PackrinthError::FailedToWriteToZip(what_to_write) => (format!("failed to write {what_to_write} to zip"), file_an_issue),
            PackrinthError::FailedToGetWalkDirEntry(original_error) => (format!("failed to get entry from WalkDir: {original_error}"), file_an_issue),
            PackrinthError::FailedToStripPath(original_path_to_be_stripped) => (format!("failed to strip path {original_path_to_be_stripped}"), file_an_issue),
            PackrinthError::FailedToCopyIntoBuffer => ("failed to copy data into buffer for zip".to_string(), file_an_issue),
            PackrinthError::FailedToAddZipDir(zip_dir) => (format!("failed to add zip directory {zip_dir}"), file_an_issue),
            PackrinthError::FailedToFinishZip => ("failed to finish zip".to_string(), file_an_issue),
            PackrinthError::BranchDoesNotExist(branch) => (format!("branch {branch} doesn't exist"), "add a branch with subcommand: branch add".to_string()),
            PackrinthError::AttemptedToAddOtherModpack => ("one of the projects is another modpack".to_string(), "remove the modpack project with subcommand: project remove <MODPACK_PROJECT>".to_string()),
            PackrinthError::NoModrinthFilesFoundForProject(project) => (format!("no files found for project {project}"), "check if the project id is spelled correctly or try to remove or add project inclusions, exclusions or overrides".to_string()),
            PackrinthError::RequestFailed(url) => (format!("request to {url} failed"), format!("check your internet connection or {file_an_issue}")),
            PackrinthError::FailedToGetCurrentDirectory(original_error) => (format!("couldn't get the current directory: {original_error}"), "the current directory may not exist or you have insufficient permissions to access the current directory".to_string()),
            PackrinthError::InvalidPackFormat(used_pack_format) => (format!("pack format {used_pack_format} is not supported by this Packrinth version"), format!("please use a configuration with pack format {}", config::CURRENT_PACK_FORMAT)),
            PackrinthError::NoBranchSpecified => ("no branch specified".to_string(), "specify a branch or remove all with the --all flag".to_string()),
            PackrinthError::NoInclusionsSpecified => ("no inclusions specified".to_string(), "specify inclusions or remove all with the --all flag".to_string()),
            PackrinthError::NoExclusionsSpecified => ("no exclusions specified".to_string(), "specify exclusions or remove all with the --all flag".to_string()),
            PackrinthError::RepoIsDirtyWhileUpdating => ("git repository has uncommitted changes".to_string(), "pass the --allow-dirty flag to force updating".to_string()),
        }
    }
}
