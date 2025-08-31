//! Structs that are only used for (de)serializing JSONs associated with Modrinth.

use crate::PackrinthError;
use crate::config::{BranchConfig, IncludeOrExclude, Loader, ProjectSettings};
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest_retry::RetryTransientMiddleware;
use reqwest_retry::policies::ExponentialBackoff;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::OnceLock;
use std::time::Duration;

const MODRINTH_API_BASE_URL: &str = "https://api.modrinth.com/v2";
static CLIENT: OnceLock<ClientWithMiddleware> = OnceLock::new();
const USER_AGENT: &str = concat!(
    "Thijzert123",
    "/",
    "packrinth",
    "/",
    env!("CARGO_PKG_VERSION")
);

fn request_text<T: ToString>(api_endpoint: &T) -> Result<String, PackrinthError> {
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

    let full_url = MODRINTH_API_BASE_URL.to_string() + api_endpoint.to_string().as_str();

    let runtime = tokio::runtime::Runtime::new().expect("Failed to create runtime");
    let response = runtime
        .block_on(client.get(&full_url).send())
        .expect("Failed to get response");
    if let Ok(text) = runtime.block_on(response.text()) {
        Ok(text)
    } else {
        Err(PackrinthError::RequestFailed(full_url))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
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

#[derive(Debug, Serialize, Deserialize, Clone)]
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
    pub dependencies: Vec<VersionDependency>,
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VersionDependency {
    pub project_id: Option<String>,
    pub dependency_type: VersionDependencyType,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum VersionDependencyType {
    #[serde(rename = "required")]
    Required,

    #[serde(rename = "optional")]
    Optional,

    #[serde(rename = "incompatible")]
    Incompatible,

    #[serde(rename = "embedded")]
    Embedded,
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
    pub dependencies: MrPackDependencies,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct File {
    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    pub project_name: String,

    pub path: String,
    pub hashes: FileHashes,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<Env>,

    pub downloads: Vec<String>,
    pub file_size: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Env {
    pub client: SideSupport,
    pub server: SideSupport,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MrPackDependencies {
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

#[derive(Debug)]
pub enum FileResult {
    Ok {
        file: File,
        dependencies: Vec<VersionDependency>,
        project_id: String,
    },
    Skipped(String),
    NotFound(String),
    Err(PackrinthError),
}

impl Project {
    pub fn from_id(id: &str) -> Result<Self, PackrinthError> {
        // Request to get general information about the project associated with the version
        let api_endpoint = format!("/project/{id}");
        let modrinth_project_response = request_text(&api_endpoint)?;
        match serde_json::from_str::<Self>(&modrinth_project_response) {
            Ok(versions) => Ok(versions),
            Err(error) => Err(PackrinthError::FailedToParseModrinthResponseJson(
                api_endpoint,
                format!("{error}"),
            )),
        }
    }
}

impl ProjectType {
    /// Returns the directory of where the project would go in to.
    pub fn directory(&self) -> Result<&str, PackrinthError> {
        match self {
            ProjectType::Mod => Ok("mods"),

            // This happens when you add a modpack as project.
            ProjectType::Modpack => Err(PackrinthError::AttemptedToAddOtherModpack),

            ProjectType::ResourcePack => Ok("resourcepack"),
            ProjectType::Shader => Ok("shader"),
        }
    }
}

impl File {
    #[must_use]
    pub fn from_project(
        branch_name: &String,
        branch_config: &BranchConfig,
        project_id: &str,
        project_settings: &ProjectSettings,
        no_alpha: bool,
        no_beta: bool,
    ) -> FileResult {
        // Handle inclusions and exclusions
        if let Some(include_or_exclude) = &project_settings.include_or_exclude {
            match include_or_exclude {
                IncludeOrExclude::Include(inclusions) => {
                    if !inclusions.contains(branch_name) {
                        return FileResult::Skipped(project_id.to_string());
                    }
                }
                IncludeOrExclude::Exclude(exclusions) => {
                    if exclusions.contains(branch_name) {
                        return FileResult::Skipped(project_id.to_string());
                    }
                }
            }
        }

        let mut loaders = Loader::modrinth_value_vec(&branch_config.acceptable_loaders);
        loaders.push(branch_config.mod_loader.modrinth_value());

        // Default loaders that will always be added
        loaders.push(Loader::Minecraft.modrinth_value());
        loaders.push(Loader::VanillaShader.modrinth_value());

        // Always add main minecraft version to acceptable minecraft versions
        let mut game_versions = vec![branch_config.minecraft_version.clone()];
        game_versions.extend(branch_config.acceptable_minecraft_versions.clone());

        let mut api_endpoint = format!(
            "/project/{project_id}/version?loaders={loaders:?}&game_versions={game_versions:?}"
        );

        // Used to know if endpoint will return ONE version or a list of versions
        let mut is_version_override = false;

        // Change endpoint to version if an override is provided for this branch
        if let Some(version_overrides) = &project_settings.version_overrides
            && let Some(version_override) = version_overrides.get(branch_name)
        {
            api_endpoint = format!("/version/{version_override}");
            is_version_override = true;
        }

        let api_response = match request_text(&api_endpoint) {
            Ok(response) => response,
            Err(error) => return FileResult::Err(error),
        };
        let modrinth_versions: Vec<Version> = if is_version_override {
            match serde_json::from_str::<Version>(&api_response) {
                Ok(version) => vec![version],
                Err(error) => {
                    return FileResult::Err(PackrinthError::FailedToParseModrinthResponseJson(
                        api_endpoint,
                        format!("{error}"),
                    ));
                }
            }
        } else {
            match serde_json::from_str(&api_response) {
                Ok(versions) => versions,
                Err(error) => {
                    return FileResult::Err(PackrinthError::FailedToParseModrinthResponseJson(
                        api_endpoint,
                        format!("{error}"),
                    ));
                }
            }
        };

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

        // If no versions were returned in the for loop.
        FileResult::NotFound(project_id.to_string())
    }

    fn from_modrinth_version(modrinth_version: &Version) -> FileResult {
        // Request to get general information about the project associated with the version
        let modrinth_project: Project = match Project::from_id(&modrinth_version.project_id) {
            Ok(versions) => versions,
            Err(error) => {
                return FileResult::Err(error);
            }
        };

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
        if primary_file_url.is_none() {
            primary_file_url = Some(&modrinth_version.files[0].url);
        }
        if primary_file_name.is_none() {
            primary_file_name = Some(&modrinth_version.files[0].filename);
        }
        if primary_file_hashes.is_none() {
            primary_file_hashes = Some(&modrinth_version.files[0].hashes);
        }
        if primary_file_size.is_none() {
            primary_file_size = Some(&modrinth_version.files[0].size);
        }

        let directory = match modrinth_project.project_type.directory() {
            Ok(directory) => directory,
            Err(error) => return FileResult::Err(error),
        };

        let path = PathBuf::from(directory)
            .join(primary_file_name.expect("No Modrinth file found"))
            .to_str()
            .expect("File name has non-valid UTF-8 characters")
            .to_string();

        FileResult::Ok {
            file: Self {
                project_name: modrinth_project.title,
                path,
                hashes: primary_file_hashes.expect("No Modrinth file found").clone(),
                env: Some(Env {
                    client: modrinth_project.client_side,
                    server: modrinth_project.server_side,
                }),
                downloads: vec![primary_file_url.expect("No Modrinth file found").clone()],
                file_size: *primary_file_size.expect("No Modrinth file found"),
            },
            dependencies: modrinth_version.dependencies.clone(),
            project_id: modrinth_project.id,
        }
    }
}
