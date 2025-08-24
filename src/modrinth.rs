//! Structs that are only used for (de)serializing JSONs associated with Modrinth.

use crate::config::Loader;
use crate::utils;
use anyhow::{Result, bail};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

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
    pub files: Vec<VersionFile>,
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
    // TODO add option to filter featured versions only (https://docs.modrinth.com/api/operations/getprojectversions)
    pub fn newest_for_project(
        project_id: &str,
        loaders: &Vec<Loader>,
        game_versions: &Vec<String>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let loaders = Loader::modrinth_value_vec(loaders);

        let endpoint = format!(
            "/project/{project_id}/version?loaders={loaders:?}&game_versions={game_versions:?}"
        );
        let api_response = utils::request_text(endpoint)?;
        let modrinth_versions: Vec<Version> = serde_json::from_str(&api_response)?;

        // Use the most recent version (index 0)
        Self::from_modrinth_version(&modrinth_versions[0])
    }

    /// Creates a <code>Version</code> by making requests to the Modrinth API.
    pub fn from_id<T: ToString>(version_id: T) -> Result<Self, Box<dyn std::error::Error>> {
        // Request to get general information about the version
        let modrinth_version_response =
            utils::request_text("/version/".to_string() + &version_id.to_string())?;
        let modrinth_version: Version = serde_json::from_str(&modrinth_version_response)?;
        Self::from_modrinth_version(&modrinth_version)
    }

    pub fn from_modrinth_version(
        modrinth_version: &Version,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        // Request to get general information about the project associated with the version
        let modrinth_project_response =
            utils::request_text("/project/".to_string() + &modrinth_version.project_id)?;
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
