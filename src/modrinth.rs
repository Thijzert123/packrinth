//! Structs that are only used for (de)serializing JSONs associated with Modrinth.

use crate::config::{BranchConfig, IncludeOrExclude, Loader, ProjectSettings};
use crate::{MRPACK_CONFIG_FILE_NAME, PackrinthError};
use serde::{Deserialize, Serialize};
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};
use std::{cmp, fs, io};
use zip::ZipArchive;
use zip::result::ZipResult;

const MODRINTH_API_BASE_URL: &str = "https://api.modrinth.com/v2";

/// Extract all the contents of a Modrinth modpack, except for the main manifest file.
///
/// # Errors
/// An [`Err`] is returned when one of these things go wrong:
/// - Failed to open file
/// - Failed to start zip archive
/// - Failed to get file by index
/// - Failed to create dirs
/// - Failed to copy file
pub fn extract_mrpack(mrpack_path: &Path, output_directory: &Path) -> ZipResult<()> {
    let zip_file = fs::File::open(mrpack_path)?;
    let mut archive = ZipArchive::new(zip_file)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let output_path = Path::new(output_directory).join(file.name());

        if file.name().ends_with('/') {
            // It's a directory
            fs::create_dir_all(&output_path)?;
        } else if file.name() != MRPACK_CONFIG_FILE_NAME {
            // Make sure parent dirs exist
            if let Some(parent) = output_path.parent()
                && !parent.exists()
            {
                fs::create_dir_all(parent)?;
            }
            // Copy file contents
            let mut output_file = fs::File::create(&output_path)?;
            io::copy(&mut file, &mut output_file)?;
        }
    }

    Ok(())
}

fn request_text<T: ToString>(api_endpoint: &T) -> Result<String, PackrinthError> {
    let full_url = MODRINTH_API_BASE_URL.to_string() + api_endpoint.to_string().as_str();
    crate::request_text(&full_url)
}

/// Part of the fields returned from the `/project` Modrinth API endpoint (v2).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub slug: String,
    pub title: String,
    pub server_side: SideSupport,
    pub client_side: SideSupport,
    pub project_type: ProjectType,
}

/// The type of project.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
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

/// The support for a specific environment (server or client).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SideSupport {
    #[serde(rename = "required")]
    Required,

    #[serde(rename = "optional")]
    Optional,

    #[serde(rename = "unsupported")]
    Unsupported,
}

/// Part of the fields returned from the `/version` Modrinth API endpoint (v2).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Version {
    pub id: String,
    pub project_id: String,
    pub version_type: VersionType,
    pub game_versions: Vec<String>,
    pub files: Vec<VersionFile>,
    pub dependencies: Vec<VersionDependency>,
}

/// Type of version.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VersionType {
    #[serde(rename = "release")]
    Release,

    #[serde(rename = "beta")]
    Beta,

    #[serde(rename = "alpha")]
    Alpha,
}

/// File in a version.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct VersionFile {
    pub url: String,
    pub filename: String,
    pub primary: bool,
    pub size: u64,
    pub hashes: FileHashes,
}

/// Hashes for a file.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FileHashes {
    pub sha1: String,
    pub sha512: String,
}

/// Dependency for a version.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct VersionDependency {
    pub project_id: Option<String>,
    pub dependency_type: VersionDependencyType,
}

/// Type of version dependency.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
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

/// The main index file in a Modrinth modpack.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
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

/// A file in a modpack.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
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

/// Environment information for a file in a Modrinth modpack.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Env {
    pub client: SideSupport,
    pub server: SideSupport,
}

/// Dependencies for a modpack, which are mod loaders that get installed alongside the modpack.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
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

/// The result of creating a file.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FileResult {
    Ok {
        file: File,
        dependencies: Vec<VersionDependency>,
        project_id: String,
    },
    Skipped,
    NotFound,
    Err(PackrinthError),
}

impl Project {
    /// Gets a project from the Modrinth ID.
    ///
    /// # Errors
    /// - [`PackrinthError::RequestFailed`] if the Modrinth request failed
    /// - [`PackrinthError::FailedToParseConfigJson`] if the Modrinth response was invalid
    pub fn from_id(id: &str) -> Result<Self, PackrinthError> {
        // Request to get general information about the project associated with the version
        let api_endpoint = format!("/project/{id}");
        let modrinth_project_response = request_text(&api_endpoint)?;
        match serde_json::from_str::<Self>(&modrinth_project_response) {
            Ok(versions) => Ok(versions),
            Err(error) => Err(PackrinthError::FailedToParseModrinthResponseJson {
                modrinth_endpoint: api_endpoint,
                error_message: error.to_string(),
            }),
        }
    }
}

impl ProjectType {
    /// Returns the directory name of where the project would go in to.
    ///
    /// # Errors
    /// - [`PackrinthError::AttemptedToAddOtherModpack`] when [`ProjectType`] is [`ProjectType::Modpack`]
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

impl Version {
    /// Fetches a [`Version`] from a sha512 hash.
    ///
    /// # Errors
    /// - [`PackrinthError::FailedToParseModrinthResponseJson`] if the response was invalid
    pub fn from_sha512_hash(hash: &str) -> Result<Self, PackrinthError> {
        let api_endpoint = format!("/version_file/{hash}?algorithm=sha512");
        let api_response = request_text(&api_endpoint)?;

        match serde_json::from_str::<Self>(&api_response) {
            Ok(versions) => Ok(versions),
            Err(error) => Err(PackrinthError::FailedToParseModrinthResponseJson {
                modrinth_endpoint: api_endpoint,
                error_message: error.to_string(),
            }),
        }
    }
}

impl MrPack {
    /// Creates a [`MrPack`] instance from a `.mrpack` file location.
    ///
    /// # Errors
    /// - [`PackrinthError::FailedToInitializeFileType`] when creating the `.mrpack` file type failed
    /// - [`PackrinthError::FailedToCreateZipArchive`] when creating the zip archive failed
    /// - [`PackrinthError::InvalidMrPack`] when the modpack doesn't fully adhere to the mrpack specifications
    /// - [`PackrinthError::FailedToReadToString`] when reading the main config in the zip failed
    pub fn from_mrpack(mrpack_path: &Path) -> Result<Self, PackrinthError> {
        let mrpack_file = match fs::File::open(mrpack_path) {
            Ok(mrpack_file) => mrpack_file,
            Err(error) => {
                return Err(PackrinthError::FailedToInitializeFileType {
                    file_to_create: mrpack_path.display().to_string(),
                    error_message: error.to_string(),
                });
            }
        };
        let mut zip_archive = match ZipArchive::new(BufReader::new(mrpack_file)) {
            Ok(zip_archive) => zip_archive,
            Err(error) => {
                return Err(PackrinthError::FailedToCreateZipArchive {
                    zip_path: mrpack_path.display().to_string(),
                    error_message: error.to_string(),
                });
            }
        };

        let mut mrpack_config_file = match zip_archive.by_name(MRPACK_CONFIG_FILE_NAME) {
            Ok(mrpack_config_file_name) => mrpack_config_file_name,
            Err(error) => {
                return Err(PackrinthError::InvalidMrPack {
                    mrpack_path: mrpack_path.display().to_string(),
                    error_message: error.to_string(),
                });
            }
        };
        let mut mrpack = String::new();
        if let Err(error) = mrpack_config_file.read_to_string(&mut mrpack) {
            return Err(PackrinthError::FailedToReadToString {
                path_to_read: mrpack_config_file.name().to_string(),
                error_message: error.to_string(),
            });
        }

        match serde_json::from_str(&mrpack) {
            Ok(mrpack) => Ok(mrpack),
            Err(error) => Err(PackrinthError::InvalidMrPack {
                mrpack_path: mrpack_path.display().to_string(),
                error_message: error.to_string(),
            }),
        }
    }
}

impl File {
    /// Creates a file type from a project.
    #[must_use]
    pub fn from_project(
        branch_name: &str,
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
                    if !inclusions.contains(&branch_name.to_string()) {
                        return FileResult::Skipped;
                    }
                }
                IncludeOrExclude::Exclude(exclusions) => {
                    if exclusions.contains(&branch_name.to_string()) {
                        return FileResult::Skipped;
                    }
                }
            }
        }

        let mut loaders = Loader::modrinth_value_vec(&branch_config.acceptable_loaders);
        if let Some(mod_loader) = &branch_config.mod_loader {
            loaders.push(mod_loader.modrinth_value());
        }

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
        let mut modrinth_versions: Vec<Version> = if is_version_override {
            match serde_json::from_str::<Version>(&api_response) {
                Ok(version) => vec![version],
                Err(error) => {
                    return FileResult::Err(PackrinthError::FailedToParseModrinthResponseJson {
                        modrinth_endpoint: api_endpoint,
                        error_message: error.to_string(),
                    });
                }
            }
        } else {
            match serde_json::from_str(&api_response) {
                Ok(versions) => versions,
                Err(error) => {
                    return FileResult::Err(PackrinthError::FailedToParseModrinthResponseJson {
                        modrinth_endpoint: api_endpoint,
                        error_message: error.to_string(),
                    });
                }
            }
        };

        // It is not confusing in this context.
        #[allow(clippy::items_after_statements)]
        fn max_semver(versions: &[String]) -> Option<semver::Version> {
            versions
                .iter()
                .filter_map(|s| s.parse::<semver::Version>().ok())
                .max()
        }

        modrinth_versions.sort_by(|a, b| {
            let ma = max_semver(&a.game_versions);
            let mb = max_semver(&b.game_versions);
            match (ma, mb) {
                (Some(va), Some(vb)) => cmp::Reverse(va).cmp(&cmp::Reverse(vb)),
                (Some(_), None) => cmp::Ordering::Less,
                (None, Some(_)) => cmp::Ordering::Greater,
                (None, None) => cmp::Ordering::Equal,
            }
        });
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
        FileResult::NotFound
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
            dependencies: modrinth_version.dependencies.clone(), // TODO fix clones here
            project_id: modrinth_version.project_id.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_from_modrinth_version() {
        let modrinth_version = Version {
            id: "X2hTodix".to_string(),
            project_id: "P7dR8mSH".to_string(),
            version_type: VersionType::Release,
            game_versions: vec!["1.21.8".to_string()],
            files: vec![VersionFile {
                url: "https://cdn.modrinth.com/data/P7dR8mSH/versions/X2hTodix/fabric-api-0.129.0%2B1.21.8.jar".to_string(),
                filename: "fabric-api-0.129.0+1.21.8.jar".to_string(),
                primary: true,
                size: 2_212_412,
                hashes: FileHashes { sha1: "9be74f9c3120ffb9f38df8f4164392d69e6ba84e".to_string(), sha512: "471babff84b36bd0f5051051bc192a97136ba733df6a49f222cb67a231d857eb4b1c5ec8dea605e146f49f75f800709f8836540a472fe8032f9fbd3f6690ec3d".to_string() },
            }],
            dependencies: vec![],
        };
        let file = File::from_modrinth_version(&modrinth_version);
        assert_eq!(FileResult::Ok {
            file: File {
                project_name: "Fabric API".to_string(),
                path: "mods/fabric-api-0.129.0+1.21.8.jar".to_string(),
                hashes: FileHashes { sha1: "9be74f9c3120ffb9f38df8f4164392d69e6ba84e".to_string(), sha512: "471babff84b36bd0f5051051bc192a97136ba733df6a49f222cb67a231d857eb4b1c5ec8dea605e146f49f75f800709f8836540a472fe8032f9fbd3f6690ec3d".to_string() },
                env: Some(Env {
                    client: SideSupport::Optional,
                    server: SideSupport::Optional,
                }),
                downloads: vec!["https://cdn.modrinth.com/data/P7dR8mSH/versions/X2hTodix/fabric-api-0.129.0%2B1.21.8.jar".to_string()],
                file_size: 2_212_412,
            },
            dependencies: vec![],
            project_id: "P7dR8mSH".to_string(),
        }, file);
    }
}
