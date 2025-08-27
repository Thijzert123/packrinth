#![warn(clippy::pedantic)]
#![allow(clippy::missing_errors_doc, clippy::missing_panics_doc)]
pub mod config;
pub mod modrinth;

#[derive(Debug)]
pub enum PackrinthError {
    PathIsFile(String),                  // path
    FailedToCreateDir(String),           // dir to create
    FailedToReadToString(String),        // path to read
    InvalidConfigJson(String),           // config path
    InvalidModrinthResponseJson(String), // modrinth endpoint
    FailedToSerialize,
    ProjectIsNotAdded(String),            // project
    OverrideDoesNotExist(String, String), // project, branch
    NoOverridesForProject(String),        // project
    NoExclusionsForProject(String),       // project
    NoInclusionsForProject(String),       // project
    ProjectAlreadyHasExclusions(String),  // project
    ProjectAlreadyHasInclusions(String),  // project
    FailedToWriteFile(String),            // path to write to
    FailedToCreateFile(String),           // file to create
    DirectoryExpected(String),            // path that should have been a dir
    FailedToStartZipFile(String),         // file to start
    FailedToWriteToZip(String),           // what to write
    FailedToGetWalkDirEntry,
    FailedToStripPath(String), // original path that had to be stripped
    FailedToCopyIntoBuffer,
    FailedToAddZipDir(String), // zip path to add (is a dir)
    FailedToFinishZip,
    BranchDoesNotExist(String), // branch
    AttemptedToAddOtherModpack,
    NoModrinthFilesFoundForProject(String), // project
    RequestFailed(String),                  // url
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
            PackrinthError::InvalidConfigJson(config_path) => (format!("config file {config_path} is invalid"), "fix it according to JSON standards".to_string()),
            PackrinthError::InvalidModrinthResponseJson(modrinth_endpoint) => (format!("modrinth response from endpoint {modrinth_endpoint} is invalid"), file_an_issue),
            PackrinthError::FailedToSerialize => ("failed to serialize to a JSON".to_string(), file_an_issue),
            PackrinthError::ProjectIsNotAdded(project) => (format!("project {project} is not added to this modpack"), "add it with subcommand: project add".to_string()),
            PackrinthError::OverrideDoesNotExist(project, branch) => (format!("{project} does not have an override for branch {branch}"), "add one with subcommand: project override add".to_string()),
            PackrinthError::NoOverridesForProject(project) => (format!("project {project} doesn't have any overrides"), "add one with subcommand: project override add".to_string()),
            PackrinthError::NoExclusionsForProject(project) => (format!("project {project} doesn't have any exclusions"), "add exclusions with subcommand: project exclude add".to_string()),
            PackrinthError::NoInclusionsForProject(project) => (format!("project {project} doesn't have any inclusions"), "add inclusions with subcommand: project include add".to_string()),
            PackrinthError::ProjectAlreadyHasExclusions(project) => (format!("project {project} already has exclusions"), "you can't have both inclusions and exclusions for one project".to_string()),
            PackrinthError::ProjectAlreadyHasInclusions(project) => (format!("project {project} already has inclusions"), "you can't have both inclusions and exclusions for one project".to_string()),
            PackrinthError::FailedToWriteFile(path_to_write_to) => (format!("failed to write to file {path_to_write_to}"), "check if you have sufficient permissions and if the file exists".to_string()),
            PackrinthError::FailedToCreateFile(file_to_create) => (format!("failed to create file {file_to_create}"), "check if you have sufficient permissions and if the path already exists".to_string()),
            PackrinthError::DirectoryExpected(path_should_have_been_dir) => (format!("expected a directory at {path_should_have_been_dir}"), "remove the path if possible".to_string()),
            PackrinthError::FailedToStartZipFile(file_to_start) => (format!("failed to start zip file at {file_to_start}"), file_an_issue),
            PackrinthError::FailedToWriteToZip(what_to_write) => (format!("failed to write {what_to_write} to zip"), file_an_issue),
            PackrinthError::FailedToGetWalkDirEntry => ("failed to get entry from WalkDir".to_string(), file_an_issue),
            PackrinthError::FailedToStripPath(original_path_to_be_stripped) => (format!("failed to strip path {original_path_to_be_stripped}"), file_an_issue),
            PackrinthError::FailedToCopyIntoBuffer => ("failed to copy data into buffer for zip".to_string(), file_an_issue),
            PackrinthError::FailedToAddZipDir(zip_dir) => (format!("failed to add zip directory {zip_dir}"), file_an_issue),
            PackrinthError::FailedToFinishZip => ("failed to finish zip".to_string(), file_an_issue),
            PackrinthError::BranchDoesNotExist(branch) => (format!("branch {branch} doesn't exist"), "add a branch with subcommand: branch add".to_string()),
            PackrinthError::AttemptedToAddOtherModpack => ("one of the projects is another modpack".to_string(), "remove the modpack project with subcommand: project remove <MODPACK_PROJECT>".to_string()),
            PackrinthError::NoModrinthFilesFoundForProject(project) => (format!("no files found for project {project}"), "check if the project id is spelled correctly or try to remove or add project inclusions, exclusions or overrides".to_string()),
            PackrinthError::RequestFailed(url) => (format!("request to {url} failed"), format!("check your internet connection or {file_an_issue}")),
        }
    }
}
