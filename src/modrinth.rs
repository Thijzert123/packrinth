//! Structs that are only used for (de)serializing JSONs associated with Modrinth.

use crate::config::{BranchConfig, IncludeOrExclude, Loader, ProjectSettings};
use anyhow::{Result, bail};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::OnceLock;
use thiserror::Error;

const MODRINTH_API_BASE_URL: &str = "https://api.modrinth.com/v2";
static CLIENT: OnceLock<Client> = OnceLock::new();
const USER_AGENT: &str = concat!(
    "Thijzert123",
    "/",
    "packrinth",
    "/",
    env!("CARGO_PKG_VERSION")
);

pub fn request_text<T: ToString>(api_endpoint: T) -> Result<String, Box<dyn std::error::Error>> {
    let client = CLIENT.get_or_init(|| {
        Client::builder()
            .user_agent(USER_AGENT)
            .build()
            .expect("Failed to build client")
    });

    let full_url = MODRINTH_API_BASE_URL.to_string() + api_endpoint.to_string().as_str();
    let text = client.get(&full_url).send()?.text()?;
    Ok(text)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub title: String,
    pub server_side: SideSupport,
    pub client_side: SideSupport,
    pub project_type: ProjectType,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ProjectType {
    #[serde(rename = "mod")]
    Mod,

    #[serde(rename = "modpack")]
    Modpack,

    #[serde(rename = "resourcepack")]
    ResourcePack,

    #[serde(rename = "shader")]
    Shader,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SideSupport {
    #[serde(rename = "required")]
    Required,

    #[serde(rename = "optional")]
    Optional,

    #[serde(rename = "unsupported")]
    Unsupported,
}

/// Struct that can be used to deserialize Modrinth version JSONs (/version).
#[derive(Debug, Serialize, Deserialize)]
pub struct Version {
    pub id: String,
    pub project_id: String,
    pub version_number: String,
    pub version_type: VersionType,
    pub files: Vec<VersionFile>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum VersionType {
    #[serde(rename = "release")]
    Release,

    #[serde(rename = "beta")]
    Beta,

    #[serde(rename = "alpha")]
    Alpha,
}

/// File in a version.
#[derive(Debug, Serialize, Deserialize)]
pub struct VersionFile {
    pub url: String,
    pub filename: String,
    pub primary: bool,
    pub size: u64,
    pub hashes: FileHashes,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileHashes {
    pub sha1: String,
    pub sha512: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MrPack {
    pub format_version: u16,
    pub game: String,
    pub version_id: String,
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,

    pub files: Vec<File>,
    pub dependencies: Dependencies,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct File {
    pub path: String,
    pub hashes: FileHashes,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<Env>,

    pub downloads: Vec<String>,
    pub file_size: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Env {
    pub client: SideSupport,
    pub server: SideSupport,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dependencies {
    pub minecraft: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub forge: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub neoforge: Option<String>,

    #[serde(rename = "fabric-loader")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fabric_loader: Option<String>,

    #[serde(rename = "quilt-loader")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quilt_loader: Option<String>,
}

#[derive(Debug, Error)]
pub enum FileError {
    #[error("{0} was skipped")]
    Skipped(String),

    #[error("{0} not found")]
    NotFound(String),
}

impl ProjectType {
    /// Returns the directory of where the project would go in to.
    pub fn directory(&self) -> Result<&str> {
        match self {
            ProjectType::Mod => Ok("mods"),

            // This happens when you add a modpack as project.
            ProjectType::Modpack => bail!("You can't add other modpacks to this modpack"),

            ProjectType::ResourcePack => Ok("resourcepack"),
            ProjectType::Shader => Ok("shader"),
        }
    }
}

impl File {
    pub fn from_project(
        branch_name: &String,
        branch_config: &BranchConfig,
        project_id: &str,
        project_settings: &ProjectSettings,
        no_alpha: bool,
        no_beta: bool,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        // Handle inclusions and exclusions
        if let Some(include_or_exclude) = &project_settings.include_or_exclude {
            match include_or_exclude {
                // TODO rename all includes to inclusions in project
                IncludeOrExclude::Include(inclusions) => {
                    if !inclusions.contains(branch_name) {
                        return Err(FileError::Skipped(project_id.to_string()).into());
                    }
                }
                IncludeOrExclude::Exclude(exclusions) => {
                    if exclusions.contains(branch_name) {
                        return Err(FileError::Skipped(project_id.to_string()).into());
                    }
                }
            }
        }

        let loaders = Loader::modrinth_value_vec(&branch_config.acceptable_loaders);
        let game_versions = &branch_config.acceptable_minecraft_versions;
        let mut api_endpoint = format!(
            "/project/{project_id}/version?loaders={loaders:?}&game_versions={game_versions:?}"
        );

        // Change endpoint to version if an override is provided for this branch
        if let Some(version_overrides) = &project_settings.version_overrides
            && let Some(version_override) = version_overrides.get(branch_name)
        {
            api_endpoint = format!("/version/{}", version_override);
        }

        let api_response = request_text(&api_endpoint)?;
        let modrinth_versions: Vec<Version> = serde_json::from_str(&api_response)?;

        for modrinth_version in modrinth_versions {
            match modrinth_version.version_type {
                VersionType::Release => return Self::from_modrinth_version(&modrinth_version),
                VersionType::Beta => {
                    if !no_beta {
                        return Self::from_modrinth_version(&modrinth_version);
                    }
                }
                VersionType::Alpha => {
                    if !no_alpha {
                        return Self::from_modrinth_version(&modrinth_version);
                    }
                }
            }
        }

        // If no versions were returned in the for loop, return error
        Err(FileError::NotFound(project_id.to_string()).into())
    }

    fn from_modrinth_version(
        modrinth_version: &Version,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        // Request to get general information about the project associated with the version
        let modrinth_project_response =
            request_text("/project/".to_string() + &modrinth_version.project_id)?;
        let modrinth_project: Project = serde_json::from_str(&modrinth_project_response)?;

        // Get the primary file. Every version should have one.
        let mut primary_file_url = None;
        let mut primary_file_name = None;
        let mut primary_file_hashes = None;
        let mut primary_file_size = None;
        for version_file in &modrinth_version.files {
            if version_file.primary {
                primary_file_url = Some(&version_file.url);
                primary_file_name = Some(&version_file.filename);
                primary_file_hashes = Some(&version_file.hashes);
                primary_file_size = Some(&version_file.size);
                break;
            }
        }

        let path = PathBuf::from(modrinth_project.project_type.directory()?)
            .join(primary_file_name.expect("No primary file found"))
            .to_str()
            .expect("File name has non-valid UTF-8 characters")
            .to_string();

        Ok(Self {
            path,
            hashes: primary_file_hashes.expect("No primary file found").clone(),
            env: Some(Env {
                client: modrinth_project.client_side,
                server: modrinth_project.server_side,
            }),
            downloads: vec![primary_file_url.expect("No primary file found").clone()],
            file_size: *primary_file_size.expect("No primary file found"),
        })
    }
}
