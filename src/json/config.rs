use crate::json::{json_to_file, modrinth};
use crate::{PackrinthError, request};
use anyhow::{Result, bail};
use clap::Error;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Display;
use std::fs;
use std::path::PathBuf;

/// Pack format version. Can be used for checking if the user uses the right packrinth
/// version for their project.
pub const CURRENT_PACK_FORMAT: u16 = 1;

// TODO add option to filter featured versions only (https://docs.modrinth.com/api/operations/getprojectversions)
pub fn newest_version_for_project(
    project_id: String,
    loaders: Vec<String>,
    game_versions: Vec<String>,
) -> Result<Version, Box<dyn std::error::Error>> {
    let endpoint = format!(
        "/project/{project_id}/version?loaders={loaders:?}&game_versions={game_versions:?}"
    );
    let api_response = request::get_text(endpoint)?;
    let modrinth_versions: Vec<modrinth::Version> = serde_json::from_str(&api_response)?;

    // Use the most recent version (index 0)
    Version::from_modrinth_version(&modrinth_versions[0])
}

const MODPACK_CONFIG_FILE_NAME: &str = "modpack.json";

/// Config file at the root of the project. File is named <code>modpack.json</code>.
#[derive(Debug, Serialize, Deserialize)]
pub struct Modpack {
    pub pack_format: u16,
    pub name: String,
    pub summary: String,
    pub author: String,
    pub branches: Vec<String>,
    pub projects: HashMap<String, Option<Project>>, // TODO check if option here can be removed
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub version_overrides: Option<HashMap<String, String>>,

    #[serde(flatten)]
    pub include_or_existing: Option<IncludeOrExclude>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum IncludeOrExclude {
    Include(Vec<String>),
    Exclude(Vec<String>),
}

const OVERRIDES_DIR_NAME: &str = "overrides";

/// Information about a single branch, for example the Minecraft version and mod loader.
/// The configuration consists of two files, one for general information intended for the
/// user of the program to edit. The other file is filled with all the exact versions
/// used for the branch. They should only be updated via one of the commands.
#[derive(Debug, Serialize, Deserialize)]
pub struct Branch {
    pub version: String,
    pub minecraft_versions: Vec<String>,
    pub loaders: Vec<Loader>,
    pub versions: Vec<Version>,
}

const BRANCH_CONFIG_FILE_NAME: &str = "branch.json";

#[derive(Debug, Serialize, Deserialize)]
struct BranchConfig {
    pub version: String,
    pub minecraft_versions: Vec<String>,
    pub loaders: Vec<Loader>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Loader {
    // For resource packs and data packs
    #[serde(rename = "minecraft")]
    Minecraft,

    // Mods
    #[serde(rename = "fabric")]
    Fabric,
    #[serde(rename = "forge")]
    Forge,
    #[serde(rename = "neoforge")]
    NeoForge,
    #[serde(rename = "quilt")]
    Quilt,
    #[serde(rename = "babric")]
    Babric,
    #[serde(rename = "bta-babric")]
    BTABabric,
    #[serde(rename = "java-agent")]
    JavaAgent,
    #[serde(rename = "legacy-fabric")]
    LegacyFabric,
    #[serde(rename = "liteloader")]
    LiteLoader,
    #[serde(rename = "modloader")]
    RigusamisModLoader,
    #[serde(rename = "nilloader")]
    NilLoader,
    #[serde(rename = "ornithe")]
    Ornithe,
    #[serde(rename = "rift")]
    Rift,

    // Shaders
    #[serde(rename = "canvas")]
    Canvas,
    #[serde(rename = "iris")]
    Iris,
    #[serde(rename = "optifine")]
    Optifine,
    #[serde(rename = "vanilla")]
    VanillaShader,

    // Plugins
    #[serde(rename = "bukkit")]
    Bukkit,
    #[serde(rename = "folia")]
    Folia,
    #[serde(rename = "paper")]
    Paper,
    #[serde(rename = "purpur")]
    Purpur,
    #[serde(rename = "spigot")]
    Spigot,
    #[serde(rename = "sponge")]
    Sponge,

    // Proxies
    #[serde(rename = "bungeecord")]
    BungeeCord,
    #[serde(rename = "velocity")]
    Velocity,
    #[serde(rename = "waterfall")]
    Waterfall,
}

const BRANCH_VERSIONS_FILE_NAME: &str = ".branch_versions.json";

#[derive(Debug, Serialize, Deserialize)]
struct BranchVersions {
    versions: Vec<Version>,
}

/// A version that is added in a subproject.
#[derive(Debug, Serialize, Deserialize)]
pub struct Version {
    pub name: String,
    pub id: String,
    pub version: String,
    pub project_id: String,
    pub side: Side,
    pub file_url: String,
    pub file_name: String,
    pub file_sha512: String,
    pub file_size: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Side {
    Server,
    Client,
    Both,
}

impl Modpack {
    /// Tries to load the config file from the working directory.
    /// If the config file doesn't exist, a file with default configuration will be made
    /// if <code>allow_init</code> is set to <code>true</code>.
    pub fn from_working_dir(allow_init: bool) -> Result<Self> {
        match fs::read_to_string(MODPACK_CONFIG_FILE_NAME) {
            Ok(contents) => Ok(serde_json::from_str(&contents)?),
            Err(error) => {
                if error.kind() == std::io::ErrorKind::NotFound && allow_init {
                    let modpack = Self {
                        pack_format: CURRENT_PACK_FORMAT,
                        name: "My Modrinth modpack".to_string(),
                        summary: "Short summary for this modpack".to_string(),
                        author: "John Doe".to_string(),
                        branches: Vec::new(),
                        projects: HashMap::new(),
                    };
                    Self::save(&modpack)?;
                    Ok(modpack)
                } else {
                    bail!(error)
                }
            }
        }
    }

    pub fn save(&self) -> Result<()> {
        json_to_file(self, MODPACK_CONFIG_FILE_NAME)
    }
}

impl Branch {
    pub fn from_working_dir2(name: &String) -> Result<Self> {
        match fs::metadata(name) {
            Ok(metadata) => {
                if metadata.is_dir() {
                    let branch_config_path = PathBuf::from(&name).join(BRANCH_CONFIG_FILE_NAME);
                    let branch_config = match fs::read_to_string(&branch_config_path) {
                        Ok(contents) => {
                            let branch_config: BranchConfig = serde_json::from_str(&contents)?;
                            branch_config
                        }
                        Err(error) => {
                            if error.kind() == std::io::ErrorKind::NotFound {
                                Self::create_default_branch_config(branch_config_path)?
                            } else {
                                bail!(error)
                            }
                        }
                    };
                    let branch_versions_path = PathBuf::from(&name).join(BRANCH_VERSIONS_FILE_NAME);
                    let branch_versions = match fs::read_to_string(&branch_versions_path) {
                        Ok(contents) => {
                            let branch_versions: BranchVersions = serde_json::from_str(&contents)?;
                            branch_versions
                        }
                        Err(error) => {
                            if error.kind() == std::io::ErrorKind::NotFound {
                                let branch_versions = BranchVersions { versions: vec![] };
                                json_to_file(&branch_versions, &branch_versions_path)?;
                                branch_versions
                            } else {
                                bail!(error)
                            }
                        }
                    };
                    Ok(Self {
                        version: branch_config.version,
                        minecraft_versions: branch_config.minecraft_versions,
                        loaders: branch_config.loaders,
                        versions: branch_versions.versions,
                    })
                } else {
                    bail!("Branch dir is not a directory");
                }
            }
            Err(error) => bail!(error),
        }
    }

    pub fn from_working_dir(
        modpack: &mut Modpack,
        name: &String,
        allow_init: bool,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if !modpack.branches.contains(name) && allow_init {
            modpack.branches.push(name.clone());
            json_to_file(&modpack, MODPACK_CONFIG_FILE_NAME)?;
        }

        let overrides_path = PathBuf::from(&name).join(OVERRIDES_DIR_NAME);
        fs::create_dir_all(&overrides_path)?;

        let branch_config_path = PathBuf::from(&name).join(BRANCH_CONFIG_FILE_NAME);
        let branch_config = match fs::read_to_string(&branch_config_path) {
            Ok(contents) => {
                let branch_config: BranchConfig = serde_json::from_str(&contents)?;
                branch_config
            }
            Err(error) => {
                if error.kind() == std::io::ErrorKind::NotFound && allow_init {
                    let branch_config = BranchConfig {
                        version: "1.0.0-vanilla".to_string(),
                        minecraft_versions: vec!["1.21.7".to_string(), "1.21.8".to_string()],
                        loaders: vec![Loader::Minecraft, Loader::VanillaShader],
                    };
                    json_to_file(&branch_config, &branch_config_path)?;
                    branch_config
                } else {
                    return Err(Box::new(error));
                }
            }
        };
        let branch_versions_path = PathBuf::from(&name).join(BRANCH_VERSIONS_FILE_NAME);
        let branch_versions = match fs::read_to_string(&branch_versions_path) {
            Ok(contents) => {
                let branch_versions: BranchVersions = serde_json::from_str(&contents)?;
                branch_versions
            }
            Err(error) => {
                if error.kind() == std::io::ErrorKind::NotFound && allow_init {
                    let branch_versions = BranchVersions { versions: vec![] };
                    json_to_file(&branch_versions, &branch_versions_path)?;
                    branch_versions
                } else {
                    return Err(Box::new(error));
                }
            }
        };
        Ok(Self {
            version: branch_config.version,
            minecraft_versions: branch_config.minecraft_versions,
            loaders: branch_config.loaders,
            versions: branch_versions.versions,
        })
    }

    // pub fn all_from_working_dir(modpack: &mut Modpack, )

    fn create_default_branch_config(branch_config_path: PathBuf) -> Result<BranchConfig> {
        let branch_config = BranchConfig {
            version: "1.0.0-vanilla".to_string(),
            minecraft_versions: vec!["1.21.7".to_string(), "1.21.8".to_string()],
            loaders: vec![Loader::Minecraft, Loader::VanillaShader],
        };
        json_to_file(&branch_config, branch_config_path)?;
        Ok(branch_config)
    }

    fn create_default_branch_versions(branch_versions_path: PathBuf) -> Result<BranchVersions> {
        let branch_versions = BranchVersions { versions: vec![] };
        json_to_file(&branch_versions, branch_versions_path)?;
        Ok(branch_versions)
    }
}

impl Version {
    pub fn from_modrinth_version(
        modrinth_version: &modrinth::Version,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        // Request to get general information about the project associated with the version
        let modrinth_project_response =
            request::get_text("/project/".to_string() + &modrinth_version.project_id)?;
        let modrinth_project: modrinth::Project = serde_json::from_str(&modrinth_project_response)?;

        // Get the primary file. Every version should have one.
        // TODO check if every version has a primary file. If it does not, remove expect calls at the bottom of this function
        let mut primary_file_url = None;
        let mut primary_file_name = None;
        let mut primary_file_sha512 = None;
        let mut primary_file_size = None;
        for version_file in &modrinth_version.files {
            if version_file.primary {
                primary_file_url = Some(&version_file.url);
                primary_file_name = Some(&version_file.filename);
                primary_file_sha512 = Some(&version_file.hashes.sha512);
                primary_file_size = Some(&version_file.size);
                break;
            }
        }

        Ok(Self {
            name: modrinth_project.title.clone(),
            id: modrinth_version.id.clone(),
            version: modrinth_version.version_number.clone(),
            project_id: modrinth_version.project_id.clone(),
            side: modrinth_project.side(),
            file_url: primary_file_url.expect("No primary file found").clone(),
            file_name: primary_file_name.expect("No primary file found").clone(),
            file_sha512: primary_file_sha512.expect("No primary file found").clone(),
            file_size: primary_file_size.expect("No primary file found").clone(),
        })
    }

    /// Creates a <code>Version</code> by making requests to the Modrinth API.
    pub fn from_id<T: ToString>(version_id: T) -> Result<Self, Box<dyn std::error::Error>> {
        // Request to get general information about the version
        let modrinth_version_response =
            request::get_text("/version/".to_string() + &version_id.to_string())?;
        let modrinth_version: modrinth::Version = serde_json::from_str(&modrinth_version_response)?;
        Self::from_modrinth_version(&modrinth_version)
    }
}
