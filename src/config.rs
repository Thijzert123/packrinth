use crate::PackrinthError;
use crate::modrinth::{Dependencies, File, MrPack};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
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

fn json_to_file<T, P>(json_value: &T, file: P) -> Result<(), PackrinthError>
where
    T: ?Sized + Serialize + Debug,
    P: AsRef<Path>,
{
    let json = match serde_json_to_string_pretty(json_value) {
        Ok(json) => json,
        Err(_error) => return Err(PackrinthError::FailedToSerialize),
    };
    if let Err(_error) = fs::write(&file, json) {
        return Err(PackrinthError::FailedToWriteFile(
            file.as_ref().display().to_string(),
        ));
    }
    Ok(())
}

fn serde_json_to_string_pretty<T>(value: &T) -> Result<String, serde_json::Error>
where
    T: ?Sized + Serialize,
{
    let mut buf = Vec::new();
    let formatter = serde_json::ser::PrettyFormatter::with_indent(b"\t");
    let mut ser = serde_json::Serializer::with_formatter(&mut buf, formatter);
    value.serialize(&mut ser)?;

    Ok(String::from_utf8_lossy(&buf).to_string())
}

/// Config file at the root of the project. File is named <code>modpack.json</code>.
#[derive(Debug, Serialize, Deserialize)]
pub struct Modpack {
    pub pack_format: u16,
    pub name: String,
    pub summary: String,
    pub author: String,
    pub branches: Vec<String>,
    pub projects: IndexMap<String, ProjectSettings>,

    #[serde(skip)]
    pub directory: PathBuf,

    #[serde(skip)]
    pub modpack_config_path: PathBuf,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectSettings {
    // IndexMap<Branch, Project version id>
    pub version_overrides: Option<IndexMap<String, String>>,

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

    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub manual_files: Vec<File>,
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
    pub projects: Vec<BranchFilesProject>,
    pub files: Vec<File>,
}

#[derive(Debug, Serialize, Deserialize, Hash, Eq, PartialEq, Clone)]
pub struct BranchFilesProject {
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
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
    pub fn new(directory: &Path) -> Result<Self, PackrinthError> {
        match fs::metadata(directory) {
            Ok(metadata) => {
                if metadata.is_file() {
                    return Err(PackrinthError::PathIsFile(directory.display().to_string()));
                }
            }
            Err(_error) => {
                if let Err(_error) = fs::create_dir_all(directory) {
                    return Err(PackrinthError::FailedToCreateDir(
                        directory.display().to_string(),
                    ));
                }
            }
        }

        let modpack = Self {
            pack_format: CURRENT_PACK_FORMAT,
            name: "My Modrinth modpack".to_string(),
            summary: "Short summary for this modpack".to_string(),
            author: "John Doe".to_string(),
            branches: Vec::new(),
            projects: IndexMap::new(),
            directory: PathBuf::from(directory),
            modpack_config_path: directory.join(MODPACK_CONFIG_FILE_NAME),
        };

        Ok(modpack)
    }

    pub fn from_directory(directory: &Path) -> Result<Self, PackrinthError> {
        let modpack_config_path = directory.join(MODPACK_CONFIG_FILE_NAME);

        let Ok(config) = fs::read_to_string(&modpack_config_path) else {
            return Err(PackrinthError::FailedToReadToString(
                modpack_config_path.display().to_string(),
            ));
        };

        let mut modpack: Modpack = match serde_json::from_str(&config) {
            Ok(modpack) => modpack,
            Err(error) => {
                return Err(PackrinthError::FailedToParseConfigJson(
                    modpack_config_path.display().to_string(),
                    format!("{error}"),
                ));
            }
        };

        modpack.directory = PathBuf::from(directory);
        modpack.modpack_config_path = modpack_config_path;

        Ok(modpack)
    }

    pub fn add_projects(
        &mut self,
        projects: &[String],
        version_overrides: &Option<IndexMap<String, String>>,
        include_or_exclude: &Option<IncludeOrExclude>,
    ) {
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
    }

    pub fn add_project_override(
        &mut self,
        project: &str,
        branch: &str,
        project_version_id: &str,
    ) -> Result<(), PackrinthError> {
        let Some(project_settings) = self.projects.get_mut(project) else {
            return Err(PackrinthError::ProjectIsNotAdded(project.to_string()));
        };

        if let Some(version_overrides) = &mut project_settings.version_overrides {
            version_overrides.insert(branch.to_string(), project_version_id.to_string());
        } else {
            project_settings.version_overrides = Some(IndexMap::from([(
                branch.to_string(),
                project_version_id.to_string(),
            )]));
        }

        Ok(())
    }

    pub fn remove_project_override(
        &mut self,
        project: &str,
        branch: &str,
    ) -> Result<(), PackrinthError> {
        let Some(project_settings) = self.projects.get_mut(project) else {
            return Err(PackrinthError::ProjectIsNotAdded(project.to_string()));
        };

        if let Some(version_overrides) = &mut project_settings.version_overrides {
            // shift_remove to show Git that one line was removed
            if version_overrides.shift_remove(branch).is_none() {
                Err(PackrinthError::OverrideDoesNotExist(
                    project.to_string(),
                    branch.to_string(),
                ))
            } else {
                Ok(())
            }
        } else {
            Err(PackrinthError::NoOverridesForProject(project.to_string()))
        }
    }

    pub fn remove_all_project_overrides(&mut self, project: &str) -> Result<(), PackrinthError> {
        if let Some(project_settings) = self.projects.get_mut(project) {
            project_settings.version_overrides = None;
            Ok(())
        } else {
            Err(PackrinthError::ProjectIsNotAdded(project.to_string()))
        }
    }

    pub fn add_project_inclusions(
        &mut self,
        project: &str,
        new_inclusions: &[String],
    ) -> Result<(), PackrinthError> {
        let Some(project_settings) = self.projects.get_mut(project) else {
            return Err(PackrinthError::ProjectIsNotAdded(project.to_string()));
        };

        if let Some(include_or_exclude) = &mut project_settings.include_or_exclude {
            if let IncludeOrExclude::Include(inclusions) = include_or_exclude {
                for new_include in new_inclusions {
                    inclusions.push(new_include.clone());
                }
            } else {
                return Err(PackrinthError::ProjectAlreadyHasExclusions(
                    project.to_string(),
                ));
            }
        } else {
            project_settings.include_or_exclude =
                Some(IncludeOrExclude::Include(Vec::from(new_inclusions)));
        }

        Ok(())
    }

    pub fn remove_project_inclusions(
        &mut self,
        project: &str,
        inclusions_to_remove: &[String],
    ) -> Result<(), PackrinthError> {
        let Some(project_settings) = self.projects.get_mut(project) else {
            return Err(PackrinthError::ProjectIsNotAdded(project.to_string()));
        };

        if let Some(include_or_exclude) = &mut project_settings.include_or_exclude
            && let IncludeOrExclude::Include(inclusions) = include_or_exclude
        {
            inclusions.retain(|x| !inclusions_to_remove.contains(x));
            Ok(())
        } else {
            Err(PackrinthError::NoInclusionsForProject(project.to_string()))
        }
    }

    pub fn remove_all_project_inclusions(&mut self, project: &str) -> Result<(), PackrinthError> {
        if let Some(project_settings) = self.projects.get_mut(project)
            && let Some(include_or_exclude) = &project_settings.include_or_exclude
        {
            // Safety check to see if the user accidentally typed include instead of exclude
            if let IncludeOrExclude::Include(_) = include_or_exclude {
                project_settings.include_or_exclude = None;
                Ok(())
            } else {
                Err(PackrinthError::NoInclusionsForProject(project.to_string()))
            }
        } else {
            Err(PackrinthError::ProjectIsNotAdded(project.to_string()))
        }
    }

    pub fn add_project_exclusions(
        &mut self,
        project: &str,
        new_exclusions: &[String],
    ) -> Result<(), PackrinthError> {
        let Some(project_settings) = self.projects.get_mut(project) else {
            return Err(PackrinthError::ProjectIsNotAdded(project.to_string()));
        };

        if let Some(include_or_exclude) = &mut project_settings.include_or_exclude {
            if let IncludeOrExclude::Exclude(exclusions) = include_or_exclude {
                for new_exclude in new_exclusions {
                    exclusions.push(new_exclude.clone());
                }
            } else {
                return Err(PackrinthError::ProjectAlreadyHasInclusions(
                    project.to_string(),
                ));
            }
        } else {
            project_settings.include_or_exclude =
                Some(IncludeOrExclude::Exclude(Vec::from(new_exclusions)));
        }

        Ok(())
    }

    pub fn remove_project_exclusions(
        &mut self,
        project: &str,
        exclusions_to_remove: &[String],
    ) -> Result<(), PackrinthError> {
        let Some(project_settings) = self.projects.get_mut(project) else {
            return Err(PackrinthError::ProjectIsNotAdded(project.to_string()));
        };

        if let Some(include_or_exclude) = &mut project_settings.include_or_exclude
            && let IncludeOrExclude::Exclude(exclusions) = include_or_exclude
        {
            exclusions.retain(|x| !exclusions_to_remove.contains(x));
            Ok(())
        } else {
            Err(PackrinthError::NoExclusionsForProject(project.to_string()))
        }
    }

    pub fn remove_all_project_exclusions(&mut self, project: &str) -> Result<(), PackrinthError> {
        if let Some(project_settings) = self.projects.get_mut(project)
            && let Some(include_or_exclude) = &project_settings.include_or_exclude
        {
            // Safety check to see if the user accidentally typed exclude instead of include
            if let IncludeOrExclude::Exclude(_) = include_or_exclude {
                project_settings.include_or_exclude = None;
                Ok(())
            } else {
                Err(PackrinthError::NoExclusionsForProject(project.to_string()))
            }
        } else {
            Err(PackrinthError::ProjectIsNotAdded(project.to_string()))
        }
    }

    pub fn remove_projects(&mut self, projects: &[String]) {
        for project in projects {
            // shift_remove to show Git that one line was removed
            self.projects.shift_remove(&String::from(project));
        }
    }

    /// Creates new branches.
    /// If it already exists, it just returns the existing branch.
    pub fn new_branch(&mut self, name: &String) -> Result<BranchConfig, PackrinthError> {
        if !self.branches.contains(name) {
            self.branches.push(name.clone());
        }
        let branch_dir = self.directory.join(name);
        if let Ok(exists) = fs::exists(&branch_dir)
            && !exists
            && let Err(_error) = fs::create_dir(&branch_dir)
        {
            return Err(PackrinthError::FailedToCreateDir(
                branch_dir.display().to_string(),
            ));
        }
        BranchConfig::from_directory(&self.directory, name)
    }

    pub fn remove_branches(&mut self, branch_names: &Vec<String>) {
        for branch_name in branch_names {
            let branch_path = self.directory.join(branch_name);

            if self.branches.contains(branch_name) {
                self.branches.retain(|x| x != branch_name);
                if let Ok(exists) = fs::exists(&branch_path)
                    && exists
                {
                    // We don't care if the dir gets removed, it is just nice to have.
                    let _ = fs::remove_dir_all(&branch_path);
                }
            }
        }
    }

    pub fn save(&self) -> Result<(), PackrinthError> {
        json_to_file(self, &self.modpack_config_path)
    }

    pub fn export_branch(&self, branch: &String) -> Result<PathBuf, PackrinthError> {
        let branch_config = BranchConfig::from_directory(&self.directory, branch)?;
        let branch_files = BranchFiles::from_directory(&self.directory, branch)?;

        let mrpack_file_name = format!("{}_{}.mrpack", self.name, branch_config.version);
        let branch_dir = self.directory.join(branch);
        let mrpack_path = branch_dir.join(&mrpack_file_name);

        let mrpack = MrPack {
            format_version: MODRINTH_PACK_FORMAT,
            game: GAME.to_string(),
            version_id: branch_config.version.clone(),
            name: self.name.clone(),
            summary: Some(self.summary.clone()),
            files: branch_files.files,
            dependencies: Self::create_dependencies(branch_config),
        };

        let mrpack_json = match serde_json_to_string_pretty(&mrpack) {
            Ok(mrpack_json) => mrpack_json,
            Err(_error) => return Err(PackrinthError::FailedToSerialize),
        };
        let options = SimpleFileOptions::default();
        let zip_file = match fs::File::create(&mrpack_path) {
            Ok(zip_file) => zip_file,
            Err(_error) => return Err(PackrinthError::FailedToInitializeFileType(mrpack_path.display().to_string())),
        };

        let mut zip = ZipWriter::new(zip_file);
        if let Err(_error) = zip.start_file(MRPACK_CONFIG_FILE_NAME, options) {
            return Err(PackrinthError::FailedToStartZipFile(
                MRPACK_CONFIG_FILE_NAME.to_string(),
            ));
        }
        if let Err(_error) = zip.write_all(mrpack_json.as_bytes()) {
            return Err(PackrinthError::FailedToWriteToZip(mrpack_json));
        }

        // If some items are skipped in the loop, this is set to Err, and it will be returned at the end.
        let mut result = Ok(());

        // Loop every file/dir in the override dirs
        for override_dir in OVERRIDE_DIRS {
            let override_dir_path = branch_dir.join(override_dir);

            // Skip override dir if it doesn't exist.
            if let Ok(exists) = fs::exists(&override_dir_path) && !exists {
                continue;
            }

            for entry in WalkDir::new(override_dir_path) {
                let entry = match entry {
                    Ok(entry) => entry,
                    Err(error) => {
                        result = Err(PackrinthError::FailedToGetWalkDirEntry(format!("{error}")));
                        continue;
                    }
                };
                // The actual path on the file system
                let path = entry.path();
                // The path the file will be in the zip (/ being the root of the zip)
                let zip_path = if let Ok(stripped_path) = path.strip_prefix(&branch_dir)
                    && let Some(zip_path) = stripped_path.to_str()
                {
                    zip_path
                } else {
                    result = Err(PackrinthError::FailedToStripPath(
                        path.display().to_string(),
                    ));
                    continue;
                };

                if path.is_file() {
                    if let Err(_error) = zip.start_file(zip_path, options) {
                        result = Err(PackrinthError::FailedToStartZipFile(zip_path.to_string()));
                        continue;
                    }
                    let mut buffer = Vec::new();
                    let mut original_file = match fs::File::open(path) {
                        Ok(file) => file,
                        Err(_error) => {
                            result = Err(PackrinthError::FailedToInitializeFileType(
                                path.display().to_string(),
                            ));
                            continue;
                        }
                    };
                    if let Err(_error) = io::copy(&mut original_file, &mut buffer) {
                        result = Err(PackrinthError::FailedToCopyIntoBuffer);
                        continue;
                    }
                    if let Err(_error) = zip.write_all(&buffer) {
                        result = Err(PackrinthError::FailedToWriteToZip(
                            String::from_utf8_lossy(&buffer).to_string(),
                        ));
                    }
                } else if path.is_dir()
                    && let Err(_error) = zip.add_directory(zip_path, options)
                {
                    result = Err(PackrinthError::FailedToAddZipDir(zip_path.to_string()));
                }
            }
        }

        if let Err(_error) = zip.finish() {
            result = Err(PackrinthError::FailedToFinishZip);
        }

        match result {
            Ok(()) => Ok(mrpack_path),
            Err(error) => Err(error),
        }
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
    pub fn from_directory(directory: &Path, name: &String) -> Result<Self, PackrinthError> {
        let branch_dir = directory.join(name);
        match fs::metadata(&branch_dir) {
            Ok(metadata) => {
                if metadata.is_dir() {
                    let branch_config_path = branch_dir.join(BRANCH_CONFIG_FILE_NAME);
                    let branch_config = match fs::read_to_string(&branch_config_path) {
                        Ok(contents) => {
                            let branch_config: Self = match serde_json::from_str(&contents) {
                                Ok(contents) => contents,
                                Err(error) => {
                                    return Err(PackrinthError::FailedToParseConfigJson(
                                        branch_config_path.display().to_string(),
                                        format!("{error}"),
                                    ));
                                }
                            };
                            branch_config
                        }
                        Err(error) => {
                            if error.kind() == io::ErrorKind::NotFound {
                                Self::create_default_branch_config(&branch_config_path)?
                            } else {
                                return Err(PackrinthError::FailedToReadToString(
                                    branch_config_path.display().to_string(),
                                ));
                            }
                        }
                    };
                    Ok(branch_config)
                } else {
                    Err(PackrinthError::DirectoryExpected(
                        branch_dir.display().to_string(),
                    ))
                }
            }
            Err(_error) => Err(PackrinthError::BranchDoesNotExist(name.clone())),
        }
    }

    fn create_default_branch_config(branch_config_path: &PathBuf) -> Result<Self, PackrinthError> {
        let branch_config = Self {
            version: "1.0.0-fabric".to_string(),
            main_minecraft_version: "1.21.8".to_string(),
            acceptable_minecraft_versions: vec!["1.21.7".to_string(), "1.21.8".to_string()],
            main_mod_loader: MainLoader::Fabric,
            loader_version: "0.17.2".to_string(),
            acceptable_loaders: vec![Loader::Minecraft, Loader::VanillaShader, Loader::Fabric],
            manual_files: vec![],
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
        if self.manual_files.is_empty() {
            println!("  - No manual files are added");
        } else {
            println!("  - Has manual files added, see the configuration file");
        }
    }
}

impl BranchFiles {
    pub fn from_directory(directory: &Path, name: &String) -> Result<Self, PackrinthError> {
        let branch_dir = directory.join(name);
        match fs::metadata(&branch_dir) {
            Ok(metadata) => {
                if metadata.is_dir() {
                    let branch_files_path = branch_dir.join(BRANCH_FILES_FILE_NAME);
                    let branch_files = match fs::read_to_string(&branch_files_path) {
                        Ok(contents) => {
                            let branch_files: Self = match serde_json::from_str(&contents) {
                                Ok(contents) => contents,
                                Err(error) => {
                                    return Err(PackrinthError::FailedToParseConfigJson(
                                        branch_files_path.display().to_string(),
                                        format!("{error}"),
                                    ));
                                }
                            };
                            branch_files
                        }
                        Err(error) => {
                            if error.kind() == io::ErrorKind::NotFound {
                                Self::default(directory, name)?
                            } else {
                                return Err(PackrinthError::FailedToReadToString(
                                    branch_files_path.display().to_string(),
                                ));
                            }
                        }
                    };
                    Ok(Self {
                        info: BRANCH_FILES_INFO.to_string(),
                        projects: branch_files.projects,
                        files: branch_files.files,
                    })
                } else {
                    Err(PackrinthError::DirectoryExpected(
                        branch_dir.display().to_string(),
                    ))
                }
            }
            Err(_error) => Err(PackrinthError::BranchDoesNotExist(name.clone())),
        }
    }

    pub fn default(directory: &Path, name: &String) -> Result<Self, PackrinthError> {
        let branch_files = Self {
            info: BRANCH_FILES_INFO.to_string(),
            projects: vec![],
            files: vec![],
        };
        branch_files.save(directory, name)?;
        Ok(branch_files)
    }

    pub fn save(&self, directory: &Path, name: &String) -> Result<(), PackrinthError> {
        let branch_files_path = directory.join(name).join(BRANCH_FILES_FILE_NAME);
        json_to_file(self, branch_files_path)
    }
}

impl PartialEq<String> for BranchFilesProject {
    fn eq(&self, other: &String) -> bool {
        if let Some(id) = &self.id {
            *id == *other
        } else {
            false
        }
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
