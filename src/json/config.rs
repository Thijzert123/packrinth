use crate::json::{json_to_file, modrinth};
use crate::request;
use anyhow::{Context, Result, bail};
use dialoguer::Confirm;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env::current_dir;
use std::fmt::Display;
use std::fs;
use std::path::{Path, PathBuf};

/// Pack format version. Can be used for checking if the user uses the right packrinth
/// version for their project.
pub const CURRENT_PACK_FORMAT: u16 = 1;

// TODO add option to filter featured versions only (https://docs.modrinth.com/api/operations/getprojectversions)
pub fn newest_version_for_project(
    project_id: &str,
    loaders: &Vec<String>,
    game_versions: &Vec<String>,
) -> Result<Version, Box<dyn std::error::Error>> {
    let endpoint = format!(
        "/project/{project_id}/version?loaders={loaders:?}&game_versions={game_versions:?}"
    );
    let api_response = request::get_text(endpoint)?;
    let modrinth_versions: Vec<modrinth::Version> = serde_json::from_str(&api_response)?;

    // Use the most recent version (index 0)
    Version::from_modrinth_version(&modrinth_versions[0])
}

pub const MODPACK_CONFIG_FILE_NAME: &str = "modpack.json";

/// Config file at the root of the project. File is named <code>modpack.json</code>.
#[derive(Debug, Serialize, Deserialize)]
pub struct Modpack {
    pub pack_format: u16,
    pub name: String,
    pub summary: String,
    pub author: String,
    pub branches: Vec<String>,
    pub projects: HashMap<String, ProjectSettings>,

    #[serde(skip)]
    pub directory: PathBuf,

    #[serde(skip)]
    pub modpack_config_path: PathBuf,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectSettings {
    // HashMap<Minecraft version, Project version id>
    #[serde(flatten)]
    pub version_overrides: Option<HashMap<String, String>>,

    #[serde(flatten)]
    pub include_or_exclude: Option<IncludeOrExclude>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum IncludeOrExclude {
    #[serde(rename = "include")]
    Include(Vec<String>),

    #[serde(rename = "exclude")]
    Exclude(Vec<String>),
}

const OVERRIDES_DIR_NAME: &str = "overrides";

/// Information about a single branch, for example the Minecraft version and mod loader.
/// The configuration consists of two files, one for general information intended for the
/// user of the program to edit. The other file is filled with all the exact versions
/// used for the branch. They should only be updated via one of the commands.
#[derive(Debug, Serialize, Deserialize)]
pub struct Branch {
    pub name: String,
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

#[allow(clippy::enum_variant_names)]
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
    RisugamisModLoader,
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
    pub fn new(directory: &Path) -> Result<Self> {
        match fs::metadata(directory) {
            Ok(metadata) => {
                if metadata.is_file() {
                    bail!(
                        "Given path {} is a file, not a directory",
                        directory.display()
                    )
                }
            }
            Err(_) => fs::create_dir_all(directory)?,
        }

        let modpack = Self {
            pack_format: CURRENT_PACK_FORMAT,
            name: "My Modrinth modpack".to_string(),
            summary: "Short summary for this modpack".to_string(),
            author: "John Doe".to_string(),
            branches: Vec::new(),
            projects: HashMap::new(),
            directory: PathBuf::from(directory),
            modpack_config_path: directory.join(MODPACK_CONFIG_FILE_NAME),
        };

        json_to_file(&modpack, &modpack.modpack_config_path)?;
        Ok(modpack)
    }

    pub fn from_directory(directory: &Path) -> Result<Self> {
        let modpack_config_path = directory.join(MODPACK_CONFIG_FILE_NAME);

        let mut modpack: Modpack = serde_json::from_str(&fs::read_to_string(&modpack_config_path)?)?;
        modpack.directory = PathBuf::from(directory);
        modpack.modpack_config_path = modpack_config_path;

        Ok(modpack)
    }

    pub fn add_projects(
        &mut self,
        projects: &[String],
        version_overrides: Option<HashMap<String, String>>,
        include_or_exclude: Option<IncludeOrExclude>,
    ) -> Result<()> {
        for project in projects {
            self.projects.insert(
                String::from(project),
                if include_or_exclude.clone().is_some() {
                    ProjectSettings {
                        version_overrides: version_overrides.clone(),
                        include_or_exclude: include_or_exclude.clone(),
                    }
                } else {
                    ProjectSettings {
                        version_overrides: None,
                        include_or_exclude: None,
                    }
                },
            );
        }

        json_to_file(self, &self.modpack_config_path)
    }

    pub fn remove_projects(&mut self, projects: &[String]) -> Result<()> {
        for project in projects {
            self.projects.remove(&String::from(project));
        }

        json_to_file(self, &self.modpack_config_path)
    }

    /// Creates a new branch.
    /// If it already exists, it just returns the existing branch.
    pub fn new_branch(&mut self, name: &String) -> Result<Branch> {
        if !self.branches.contains(name) {
            self.branches.push(name.clone());
            json_to_file(self, &self.modpack_config_path)?;
        }
        let branch_dir = self.directory.join(name);
        if let Ok(exists) = fs::exists(&branch_dir)
            && !exists
        {
            fs::create_dir(&branch_dir)?;
        }
        Branch::from_directory(&self.directory, name)
    }

    pub fn remove_branches(&mut self, branch_names: &Vec<String>) -> Result<()> {
        for branch_name in branch_names {
            let branch_path = self.directory.join(branch_name);

            if self.branches.contains(branch_name) {
                self.branches.retain(|x| x != branch_name);
                if let Ok(exists) = fs::exists(&branch_path)
                    && exists
                {
                    fs::remove_dir_all(&branch_path)?;
                }
            }
            json_to_file(self, &self.modpack_config_path)?;
        }

        Ok(())
    }
}

impl Branch {
    pub fn from_directory(directory: &Path, name: &String) -> Result<Self> {
        let branch_dir = directory.join(name);
        match fs::metadata(&branch_dir).with_context(|| format!("Branch {} doesn't exist", name)) {
            Ok(metadata) => {
                if metadata.is_dir() {
                    let branch_config_path = branch_dir.join(BRANCH_CONFIG_FILE_NAME);
                    let branch_config =
                        match fs::read_to_string(&branch_config_path).with_context(|| {
                            format!("Failed to read {}", &branch_config_path.display())
                        }) {
                            Ok(contents) => {
                                let branch_config: BranchConfig = serde_json::from_str(&contents)?;
                                branch_config
                            }
                            Err(error) if error.downcast_ref::<std::io::Error>().is_some() => {
                                if error.downcast_ref::<std::io::Error>().unwrap().kind()
                                    == std::io::ErrorKind::NotFound
                                {
                                    Self::create_default_branch_config(&branch_config_path)?
                                } else {
                                    bail!(error)
                                }
                            }
                            Err(error) => bail!(error),
                        };
                    let branch_versions_path = branch_dir.join(BRANCH_VERSIONS_FILE_NAME);
                    let branch_versions = match fs::read_to_string(&branch_versions_path)
                        .with_context(|| {
                            format!("Failed to read {}", &branch_versions_path.display())
                        }) {
                        Ok(contents) => {
                            let branch_versions: BranchVersions = serde_json::from_str(&contents)?;
                            branch_versions
                        }
                        Err(error) if error.downcast_ref::<std::io::Error>().is_some() => {
                            if error.downcast_ref::<std::io::Error>().unwrap().kind()
                                == std::io::ErrorKind::NotFound
                            {
                                Self::create_default_branch_versions(&branch_versions_path)?
                            } else {
                                bail!(error)
                            }
                        }
                        Err(error) => bail!(error),
                    };
                    Ok(Self {
                        name: name.clone(),
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

    fn create_default_branch_config(branch_config_path: &PathBuf) -> Result<BranchConfig> {
        let branch_config = BranchConfig {
            version: "1.0.0-vanilla".to_string(),
            minecraft_versions: vec!["1.21.7".to_string(), "1.21.8".to_string()],
            loaders: vec![Loader::Minecraft, Loader::VanillaShader],
        };
        json_to_file(&branch_config, branch_config_path)?;
        Ok(branch_config)
    }

    fn create_default_branch_versions(branch_versions_path: &PathBuf) -> Result<BranchVersions> {
        let branch_versions = BranchVersions { versions: vec![] };
        json_to_file(&branch_versions, branch_versions_path)?;
        Ok(branch_versions)
    }
}

impl Display for Branch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Branch {}:", self.name)?;
        writeln!(f, "  - Branch version: {}", self.version)?;
        writeln!(
            f,
            "  - Acceptable Minecraft versions: {}",
            self.minecraft_versions.join(", ")
        )?;
        write!(
            f,
            "  - Acceptable loaders: {}",
            Loader::value_vec(&self.loaders).join(", ")
        )
    }
}

impl Loader {
    pub fn value_vec(loaders: &Vec<Self>) -> Vec<&str> {
        let mut values = Vec::new();
        for loader in loaders {
            values.push(loader.value());
        }
        values
    }

    fn value(&self) -> &str {
        match self {
            Loader::Minecraft => "Minecraft",
            Loader::Fabric => "Fabric",
            Loader::Forge => "Forge",
            Loader::NeoForge => "NeoForge",
            Loader::Quilt => "Quilt",
            Loader::Babric => "Babric",
            Loader::BTABabric => "BTA (Babric)",
            Loader::JavaAgent => "Java Agent",
            Loader::LegacyFabric => "Legacy Fabric",
            Loader::LiteLoader => "LiteLoader",
            Loader::RisugamisModLoader => "Risugami's ModLoader",
            Loader::NilLoader => "NilLoader",
            Loader::Ornithe => "Ornithe",
            Loader::Rift => "Rift",
            Loader::Canvas => "Canvas",
            Loader::Iris => "Iris",
            Loader::Optifine => "OptiFine",
            Loader::VanillaShader => "Vanilla Shader",
            Loader::Bukkit => "Bukkit",
            Loader::Folia => "Folia",
            Loader::Paper => "Paper",
            Loader::Purpur => "Purpur",
            Loader::Spigot => "Spigot",
            Loader::Sponge => "Sponge",
            Loader::BungeeCord => "BungeeCord",
            Loader::Velocity => "Velocity",
            Loader::Waterfall => "Waterfall",
        }
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
            file_size: *primary_file_size.expect("No primary file found"),
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
