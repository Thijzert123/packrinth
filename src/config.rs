use crate::modrinth::{Dependencies, File, MrPack};
use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Debug;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::{fs, io};
use walkdir::WalkDir;
use zip::ZipWriter;
use zip::write::SimpleFileOptions;

/// Pack format version. Can be used for checking if the user uses the right packrinth
/// version for their project.
pub const CURRENT_PACK_FORMAT: u16 = 1;

pub const MODPACK_CONFIG_FILE_NAME: &str = "modpack.json";

fn json_to_file<T, P>(json_value: &T, file: P) -> Result<()>
where
    T: ?Sized + Serialize + Debug,
    P: AsRef<Path>,
{
    let json = serde_json::to_string_pretty(json_value)
        .with_context(|| format!("Failed to serialize {json_value:?} to JSON"))?;
    fs::write(&file, json)
        .with_context(|| format!("Failed write to {}", &file.as_ref().display()))?;
    Ok(())
}

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

const BRANCH_CONFIG_FILE_NAME: &str = "branch.json";

#[derive(Debug, Serialize, Deserialize)]
pub struct BranchConfig {
    pub version: String,
    pub main_minecraft_version: String,
    pub acceptable_minecraft_versions: Vec<String>,
    pub main_mod_loader: MainLoader,
    pub loader_version: String,
    pub acceptable_loaders: Vec<Loader>,
}

/// Loaders that a launcher has to install with the modpack.
/// See <https://support.modrinth.com/en/articles/8802351-modrinth-modpack-format-mrpack>
/// at `dependencies` for more information.
#[derive(Debug, Serialize, Deserialize)]
pub enum MainLoader {
    #[serde(rename = "forge")]
    Forge,
    #[serde(rename = "neoforge")]
    NeoForge,
    #[serde(rename = "fabric")]
    Fabric,
    #[serde(rename = "quilt")]
    Quilt,
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

const BRANCH_FILES_FILE_NAME: &str = ".branch_files.json";
const BRANCH_FILES_INFO: &str = "This file is managed by Packrinth and not intended for manual editing. You should, however, add it to your Git repository.";

#[derive(Debug, Serialize, Deserialize)]
pub struct BranchFiles {
    info: String,
    pub files: Vec<File>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Side {
    Server,
    Client,
    Both,
}

/// The current most recent pack format of a .mrpack file.
const MODRINTH_PACK_FORMAT: u16 = 1;
/// The game to put in the mrpack.
const GAME: &str = "minecraft";
const MRPACK_CONFIG_FILE_NAME: &str = "modrinth.index.json";
const OVERRIDE_DIRS: [&str; 3] = ["overrides", "server-overrides", "client-overrides"];

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

        modpack.save()?;
        Ok(modpack)
    }

    pub fn from_directory(directory: &Path) -> Result<Self> {
        let modpack_config_path = directory.join(MODPACK_CONFIG_FILE_NAME);

        let mut modpack: Modpack =
            serde_json::from_str(&fs::read_to_string(&modpack_config_path)?)?;
        modpack.directory = PathBuf::from(directory);
        modpack.modpack_config_path = modpack_config_path;

        Ok(modpack)
    }

    pub fn add_projects(
        &mut self,
        projects: &[String],
        version_overrides: &Option<HashMap<String, String>>,
        include_or_exclude: &Option<IncludeOrExclude>,
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

        self.save()
    }

    pub fn add_project_override(
        &mut self,
        project: &str,
        minecraft_version: &str,
        project_version_id: &str,
    ) -> Result<()> {
        let Some(project_settings) = self.projects.get_mut(project) else {
            bail!("Project {} isn't added to this modpack", project);
        };

        if let Some(version_overrides) = &mut project_settings.version_overrides {
            version_overrides.insert(
                minecraft_version.to_string(),
                project_version_id.to_string(),
            );
        } else {
            project_settings.version_overrides = Some(HashMap::from([(
                minecraft_version.to_string(),
                project_version_id.to_string(),
            )]));
        }

        self.save()
    }

    pub fn remove_project_override(
        &mut self,
        project: &str,
        minecraft_version: &str,
    ) -> Result<()> {
        let Some(project_settings) = self.projects.get_mut(project) else {
            bail!("Project {} isn't added to this modpack", project);
        };

        if let Some(version_overrides) = &mut project_settings.version_overrides {
            if version_overrides.remove(minecraft_version).is_none() {
                bail!(
                    "No override was added for {} and Minecraft version {}",
                    project,
                    minecraft_version
                );
            }
        } else {
            bail!("Project {} doesn't have any overrides", project);
        }

        self.save()
    }

    pub fn remove_all_project_overrides(&mut self, project: &str) -> Result<()> {
        if let Some(project_settings) = self.projects.get_mut(project) {
            project_settings.version_overrides = None;
            self.save()
        } else {
            bail!("Project {} isn't added to this modpack", project);
        }
    }

    pub fn add_project_inclusions(
        &mut self,
        project: &str,
        new_inclusions: &[String],
    ) -> Result<()> {
        let Some(project_settings) = self.projects.get_mut(project) else {
            bail!("Project {} isn't added to this modpack", project);
        };

        if let Some(include_or_exclude) = &mut project_settings.include_or_exclude {
            if let IncludeOrExclude::Include(inclusions) = include_or_exclude {
                for new_include in new_inclusions {
                    inclusions.push(new_include.clone());
                }
            } else {
                bail!(
                    "Project {} already has exclusions added. You can't have both inclusions and exclusions for one project",
                    project
                );
            }
        } else {
            project_settings.include_or_exclude =
                Some(IncludeOrExclude::Include(Vec::from(new_inclusions)));
        }

        self.save()
    }

    pub fn remove_project_inclusions(
        &mut self,
        project: &str,
        inclusions_to_remove: &[String],
    ) -> Result<()> {
        let Some(project_settings) = self.projects.get_mut(project) else {
            bail!("Project {} isn't added to this modpack", project);
        };

        if let Some(include_or_exclude) = &mut project_settings.include_or_exclude
            && let IncludeOrExclude::Include(inclusions) = include_or_exclude
        {
            inclusions.retain(|x| !inclusions_to_remove.contains(x));
            self.save()
        } else {
            bail!("Project {} doesn't have any inclusions added", project);
        }
    }

    pub fn remove_all_project_inclusions(&mut self, project: &str) -> Result<()> {
        if let Some(project_settings) = self.projects.get_mut(project)
            && let Some(include_or_exclude) = &project_settings.include_or_exclude
        {
            // Safety check to see if the user accidentally typed include instead of exclude
            if let IncludeOrExclude::Include(_) = include_or_exclude {
                project_settings.include_or_exclude = None;
                self.save()
            } else {
                bail!("Project {} doesn't have inclusions added", project);
            }
        } else {
            bail!("Project {} isn't added to this modpack", project);
        }
    }

    pub fn add_project_exclusions(
        &mut self,
        project: &str,
        new_exclusions: &[String],
    ) -> Result<()> {
        let Some(project_settings) = self.projects.get_mut(project) else {
            bail!("Project {} isn't added to this modpack", project);
        };

        if let Some(include_or_exclude) = &mut project_settings.include_or_exclude {
            if let IncludeOrExclude::Exclude(exclusions) = include_or_exclude {
                for new_exclude in new_exclusions {
                    exclusions.push(new_exclude.clone());
                }
            } else {
                bail!(
                    "Project {} already has inclusions added. You can't have both inclusions and exclusions for one project",
                    project
                );
            }
        } else {
            project_settings.include_or_exclude =
                Some(IncludeOrExclude::Exclude(Vec::from(new_exclusions)));
        }

        self.save()
    }

    pub fn remove_project_exclusions(
        &mut self,
        project: &str,
        exclusions_to_remove: &[String],
    ) -> Result<()> {
        let Some(project_settings) = self.projects.get_mut(project) else {
            bail!("Project {} isn't added to this modpack", project);
        };

        if let Some(include_or_exclude) = &mut project_settings.include_or_exclude
            && let IncludeOrExclude::Exclude(exclusions) = include_or_exclude
        {
            exclusions.retain(|x| !exclusions_to_remove.contains(x));
            self.save()
        } else {
            bail!("Project {} doesn't have any exclusions added", project);
        }
    }

    pub fn remove_all_project_exclusions(&mut self, project: &str) -> Result<()> {
        if let Some(project_settings) = self.projects.get_mut(project)
            && let Some(include_or_exclude) = &project_settings.include_or_exclude
        {
            // Safety check to see if the user accidentally typed exclude instead of include
            if let IncludeOrExclude::Exclude(_) = include_or_exclude {
                project_settings.include_or_exclude = None;
                self.save()
            } else {
                bail!("Project {} doesn't have exclusions added", project);
            }
        } else {
            bail!("Project {} isn't added to this modpack", project);
        }
    }

    pub fn remove_projects(&mut self, projects: &[String]) -> Result<()> {
        for project in projects {
            self.projects.remove(&String::from(project));
        }

        self.save()
    }

    /// Creates new branches.
    /// If it already exists, it just returns the existing branch.
    pub fn new_branch(&mut self, name: &String) -> Result<BranchConfig> {
        if !self.branches.contains(name) {
            self.branches.push(name.clone());
            self.save()?;
        }
        let branch_dir = self.directory.join(name);
        if let Ok(exists) = fs::exists(&branch_dir)
            && !exists
        {
            fs::create_dir(&branch_dir)?;
        }
        BranchConfig::from_directory(&self.directory, name)
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
            self.save()?;
        }

        Ok(())
    }

    fn save(&self) -> Result<()> {
        json_to_file(self, &self.modpack_config_path)
    }

    pub fn export(&self, branch: &String) -> Result<PathBuf> {
        let branch_config = BranchConfig::from_directory(&self.directory, branch)?;
        let branch_files = BranchFiles::from_directory(&self.directory, branch)?;

        let mrpack_file_name = format!("{}_{}.mrpack", self.name, branch_config.version);
        let mrpack_path = self.directory.join(&mrpack_file_name);

        let mrpack = MrPack {
            format_version: MODRINTH_PACK_FORMAT,
            game: GAME.to_string(),
            version_id: branch_config.version.clone(),
            name: self.name.clone(),
            summary: Some(self.summary.clone()),
            files: branch_files.files,
            dependencies: Self::create_dependencies(branch_config),
        };

        let mrpack_json = serde_json::to_string_pretty(&mrpack)?;
        let options = SimpleFileOptions::default();

        let mut zip = ZipWriter::new(fs::File::create(&mrpack_path)?);
        zip.start_file(MRPACK_CONFIG_FILE_NAME, options)?;
        zip.write_all(mrpack_json.as_bytes())?;

        let branch_dir = self.directory.join(branch);
        // Loop every file/dir in the override dirs
        for override_dir in OVERRIDE_DIRS {
            for entry in WalkDir::new(branch_dir.join(override_dir)) {
                let entry = entry?;
                // The actual path on the file system
                let path = entry.path();
                // The path the file will be in the zip (/ being the root of the zip)
                let zip_path = path
                    .strip_prefix(&branch_dir)?
                    .to_str()
                    .expect("Couldn't strip to zip path");

                if path.is_file() {
                    zip.start_file(zip_path, options)?;
                    let mut buffer = Vec::new();
                    io::copy(&mut fs::File::open(path)?, &mut buffer)?;
                    zip.write_all(&buffer)?;
                } else if path.is_dir() {
                    zip.add_directory(zip_path, options)?;
                }
            }
        }

        zip.finish()?;

        Ok(mrpack_path)
    }

    fn create_dependencies(branch_config: BranchConfig) -> Dependencies {
        let mut forge = None;
        let mut neoforge = None;
        let mut fabric_loader = None;
        let mut quilt_loader = None;

        match branch_config.main_mod_loader {
            MainLoader::Forge => forge = Some(branch_config.loader_version),
            MainLoader::NeoForge => neoforge = Some(branch_config.loader_version),
            MainLoader::Fabric => fabric_loader = Some(branch_config.loader_version),
            MainLoader::Quilt => quilt_loader = Some(branch_config.loader_version),
        }

        Dependencies {
            minecraft: branch_config.main_minecraft_version,
            forge,
            neoforge,
            fabric_loader,
            quilt_loader,
        }
    }
}

impl BranchConfig {
    pub fn from_directory(directory: &Path, name: &String) -> Result<Self> {
        let branch_dir = directory.join(name);
        match fs::metadata(&branch_dir).with_context(|| format!("Branch {name} doesn't exist")) {
            Ok(metadata) => {
                if metadata.is_dir() {
                    let branch_config_path = branch_dir.join(BRANCH_CONFIG_FILE_NAME);
                    let branch_config =
                        match fs::read_to_string(&branch_config_path).with_context(|| {
                            format!("Failed to read {}", &branch_config_path.display())
                        }) {
                            Ok(contents) => {
                                let branch_config: Self = serde_json::from_str(&contents)?;
                                branch_config
                            }
                            Err(error) if error.downcast_ref::<io::Error>().is_some() => {
                                if error.downcast_ref::<io::Error>().unwrap().kind()
                                    == io::ErrorKind::NotFound
                                {
                                    Self::create_default_branch_config(&branch_config_path)?
                                } else {
                                    bail!(error)
                                }
                            }
                            Err(error) => bail!(error),
                        };
                    Ok(Self {
                        version: branch_config.version,
                        main_minecraft_version: branch_config.main_minecraft_version,
                        acceptable_minecraft_versions: branch_config.acceptable_minecraft_versions,
                        main_mod_loader: branch_config.main_mod_loader,
                        loader_version: branch_config.loader_version,
                        acceptable_loaders: branch_config.acceptable_loaders,
                    })
                } else {
                    bail!("Branch dir is not a directory");
                }
            }
            Err(error) => bail!(error),
        }
    }

    fn create_default_branch_config(branch_config_path: &PathBuf) -> Result<Self> {
        let branch_config = Self {
            version: "1.0.0-fabric".to_string(),
            main_minecraft_version: "1.21.8".to_string(),
            acceptable_minecraft_versions: vec!["1.21.7".to_string(), "1.21.8".to_string()],
            main_mod_loader: MainLoader::Fabric,
            loader_version: "0.17.2".to_string(),
            acceptable_loaders: vec![Loader::Minecraft, Loader::VanillaShader, Loader::Fabric],
        };
        json_to_file(&branch_config, branch_config_path)?;
        Ok(branch_config)
    }

    pub fn print_display(&self, name: &str) {
        println!("Branch {name}:");
        println!("  - Branch version: {}", self.version);
        println!(
            "  - Main Minecraft version: {}",
            self.main_minecraft_version
        );
        println!(
            "  - Acceptable Minecraft versions: {}",
            self.acceptable_minecraft_versions.join(", ")
        );
        println!("  - Main mod loader: {}", self.main_mod_loader.value());
        println!(
            "  - Acceptable loaders: {}",
            Loader::pretty_value_vec(&self.acceptable_loaders).join(", ")
        );
    }
}

impl BranchFiles {
    pub fn from_directory(directory: &Path, name: &String) -> Result<Self> {
        let branch_dir = directory.join(name);
        match fs::metadata(&branch_dir).with_context(|| format!("Branch {name} doesn't exist")) {
            Ok(metadata) => {
                if metadata.is_dir() {
                    let branch_files_path = branch_dir.join(BRANCH_FILES_FILE_NAME);
                    let branch_files = match fs::read_to_string(&branch_files_path)
                        .with_context(|| format!("Failed to read {}", &branch_files_path.display()))
                    {
                        Ok(contents) => {
                            let branch_files: Self = serde_json::from_str(&contents)?;
                            branch_files
                        }
                        Err(error) if error.downcast_ref::<io::Error>().is_some() => {
                            if error.downcast_ref::<io::Error>().unwrap().kind()
                                == io::ErrorKind::NotFound
                            {
                                Self::create_default_branch_files(&branch_files_path)?
                            } else {
                                bail!(error)
                            }
                        }
                        Err(error) => bail!(error),
                    };
                    Ok(Self {
                        info: BRANCH_FILES_INFO.to_string(),
                        files: branch_files.files,
                    })
                } else {
                    bail!("Branch dir is not a directory");
                }
            }
            Err(error) => bail!(error),
        }
    }

    pub fn save(&self, directory: &Path, name: &String) -> Result<()> {
        let branch_files_path = directory.join(name).join(BRANCH_FILES_FILE_NAME);
        json_to_file(self, branch_files_path)
    }

    fn create_default_branch_files(branch_versions_path: &PathBuf) -> Result<Self> {
        let branch_versions = Self {
            info: BRANCH_FILES_INFO.to_string(),
            files: vec![],
        };
        json_to_file(&branch_versions, branch_versions_path)?;
        Ok(branch_versions)
    }
}

impl MainLoader {
    const fn value(&self) -> &str {
        match self {
            MainLoader::Forge => "Forge",
            MainLoader::NeoForge => "NeoForge",
            MainLoader::Fabric => "Fabric",
            MainLoader::Quilt => "Quilt",
        }
    }
}

impl Loader {
    #[must_use]
    pub fn pretty_value_vec(loaders: &Vec<Self>) -> Vec<&str> {
        let mut values = Vec::new();
        for loader in loaders {
            values.push(loader.pretty_value());
        }
        values
    }

    #[must_use]
    pub fn modrinth_value_vec(loaders: &Vec<Self>) -> Vec<&str> {
        let mut values = Vec::new();
        for loader in loaders {
            values.push(loader.modrinth_value());
        }
        values
    }

    const fn pretty_value(&self) -> &str {
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

    const fn modrinth_value(&self) -> &str {
        match self {
            Loader::Minecraft => "minecraft",
            Loader::Fabric => "fabric",
            Loader::Forge => "forge",
            Loader::NeoForge => "neoforge",
            Loader::Quilt => "quilt",
            Loader::Babric => "babric",
            Loader::BTABabric => "bta-babric",
            Loader::JavaAgent => "java-agent",
            Loader::LegacyFabric => "legacy-fabric",
            Loader::LiteLoader => "liteloader",
            Loader::RisugamisModLoader => "modloader",
            Loader::NilLoader => "nilloader",
            Loader::Ornithe => "ornithe",
            Loader::Rift => "rift",
            Loader::Canvas => "canvas",
            Loader::Iris => "iris",
            Loader::Optifine => "optifine",
            Loader::VanillaShader => "vanilla",
            Loader::Bukkit => "bukkit",
            Loader::Folia => "folia",
            Loader::Paper => "paper",
            Loader::Purpur => "purpur",
            Loader::Spigot => "spigot",
            Loader::Sponge => "sponge",
            Loader::BungeeCord => "bungeecord",
            Loader::Velocity => "velocity",
            Loader::Waterfall => "waterfall",
        }
    }
}
