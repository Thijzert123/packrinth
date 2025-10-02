//! Structs for configuring and managing a Packrinth modpack instance.

use crate::modrinth::{File, MrPack, MrPackDependencies};
use crate::{MRPACK_CONFIG_FILE_NAME, PackrinthError};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::{fs, io};
use walkdir::WalkDir;
use zip::ZipWriter;
use zip::write::SimpleFileOptions;

/// Pack format version.
///
/// Can be used for checking if the user uses the right packrinth
/// version for their project.
pub const CURRENT_PACK_FORMAT: u16 = 1;

fn json_to_file<T, P>(json_value: &T, file: P) -> Result<(), PackrinthError>
where
    T: ?Sized + Serialize + Debug,
    P: AsRef<Path>,
{
    let json = match serde_json_to_string_pretty(json_value) {
        Ok(json) => json,
        Err(error) => {
            return Err(PackrinthError::FailedToSerialize {
                error_message: error.to_string(),
            });
        }
    };
    if let Err(error) = fs::write(&file, json) {
        return Err(PackrinthError::FailedToWriteFile {
            path_to_write_to: file.as_ref().display().to_string(),
            error_message: error.to_string(),
        });
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

/// Config file at the root of the modpack directory.
///
/// It is important to know that every function that modifies the modpack, DOESN'T save it to
/// the configuration file. To do that, use [`Modpack::save`].
#[derive(Debug, Serialize, Deserialize)]
pub struct Modpack {
    pub pack_format: u16,
    pub name: String,
    pub summary: String,
    pub author: String,
    pub require_all: bool,
    pub auto_dependencies: bool,
    pub branches: Vec<String>,

    /// A map of added projects.
    ///
    /// The key is the Modrinth project ID (fabric-api or 3jfh38sf),
    /// and the value is a map of settings for the project.
    pub projects: IndexMap<String, ProjectSettings>,

    #[serde(skip)]
    pub directory: PathBuf,

    #[serde(skip)]
    pub modpack_config_path: PathBuf,
}

/// Settings for one project that is added to a modpack.
#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectSettings {
    // IndexMap<Branch, Project version id>
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_overrides: Option<IndexMap<String, String>>,

    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_or_exclude: Option<IncludeOrExclude>,
}

/// Inclusions or exclusions for a project.
///
/// Inclusions allow projects to ONLY be added
/// to specific branches, while exclusions remove projects from branches.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum IncludeOrExclude {
    #[serde(rename = "include")]
    Include(Vec<String>),

    #[serde(rename = "exclude")]
    Exclude(Vec<String>),
}

/// The branch configuration file name.
pub const BRANCH_CONFIG_FILE_NAME: &str = "branch.json";

/// Configuration for a branch.
///
/// This configuration is supposed to be edited by the user.
#[derive(Debug, Serialize, Deserialize)]
pub struct BranchConfig {
    pub version: String,

    pub minecraft_version: String,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub acceptable_minecraft_versions: Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mod_loader: Option<MainLoader>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub loader_version: Option<String>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub acceptable_loaders: Vec<Loader>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub manual_files: Vec<File>,
}

/// Loader that a launcher has to install with the modpack.
///
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

/// All Modrinth loaders, including loaders for shader packs.
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

/// The branch files configuration file name.
pub const BRANCH_FILES_FILE_NAME: &str = ".branch_files.json";
const BRANCH_FILES_INFO: &str = "This file is managed by Packrinth and not intended for manual editing. You should, however, add it to your Git repository.";

/// A configuration file for all the files for a branch.
///
/// This configuration file is intended to be updated by Packrinth, not by theo
#[derive(Debug, Serialize, Deserialize)]
pub struct BranchFiles {
    info: String,

    /// All projects added in a branch.
    ///
    /// These can be used for generating documentation based on [`BranchFiles`]
    /// without making additional web request. That is because a [`File`] doesn't contain
    /// a human-friendly name and Modrinth ID for a project.
    pub projects: Vec<BranchFilesProject>,

    pub files: Vec<File>,
}
/// Project for [`BranchFiles`].
#[derive(Debug, Serialize, Deserialize, Clone, Hash, Eq, PartialEq)]
pub struct BranchFilesProject {
    pub name: String,

    /// The Modrinth ID for a project. If [`None`], the project was a manual project.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// The name of the modpack configuration file.
pub const MODPACK_CONFIG_FILE_NAME: &str = "modpack.json";

/// The name of the target directory
pub const TARGET_DIRECTORY: &str = "target";

/// The current most recent pack format of a .mrpack file.
const MODRINTH_PACK_FORMAT: u16 = 1;
/// The game to put in the mrpack.
const GAME: &str = "minecraft";
const OVERRIDE_DIRS: [&str; 3] = ["overrides", "server-overrides", "client-overrides"];

impl Modpack {
    /// Creates a new modpack to a directory.
    ///
    /// If the directory doesn't exist, it will be created, including its parent directories
    /// if necessary.
    ///
    /// If `force` is set to `true`, a modpack will be initialized
    /// even if one already exists in the specified directory. In this case, the configuration file
    /// will be overridden with the default configuration.
    ///
    /// # Errors
    /// - [`PackrinthError::ModpackAlreadyExists`] if a modpack configuration file already exists
    ///   in the directory and `force` was `false`
    /// - [`PackrinthError::PathIsFile`] if the directory path is a file
    /// - [`PackrinthError::ProjectIsNotAdded`] if creating the directory failed
    pub fn new(directory: &Path, force: bool) -> Result<Self, PackrinthError> {
        let modpack_config_path = directory.join(MODPACK_CONFIG_FILE_NAME);
        if !force
            && let Ok(exists) = fs::exists(&modpack_config_path)
            && exists
        {
            return Err(PackrinthError::ModpackAlreadyExists {
                directory: directory.display().to_string(),
            });
        }

        match fs::metadata(directory) {
            Ok(metadata) => {
                if metadata.is_file() {
                    return Err(PackrinthError::PathIsFile {
                        path: directory.display().to_string(),
                    });
                }
            }
            Err(_error) => {
                if let Err(error) = fs::create_dir_all(directory) {
                    return Err(PackrinthError::FailedToCreateDir {
                        dir_to_create: directory.display().to_string(),
                        error_message: error.to_string(),
                    });
                }
            }
        }

        let modpack = Self {
            directory: PathBuf::from(directory),
            modpack_config_path: directory.join(MODPACK_CONFIG_FILE_NAME),
            ..Self::default()
        };

        Ok(modpack)
    }

    /// Gets a modpack from a directory.
    ///
    /// # Errors
    /// - [`PackrinthError::FailedToReadToString`] if reading the modpack configuration failed
    /// - [`PackrinthError::FailedToParseConfigJson`] if parsing the configuration JSON failed
    pub fn from_directory(directory: &Path) -> Result<Self, PackrinthError> {
        let modpack_config_path = directory.join(MODPACK_CONFIG_FILE_NAME);

        let config = match fs::read_to_string(&modpack_config_path) {
            Ok(config) => config,
            Err(error) => {
                return Err(PackrinthError::FailedToReadToString {
                    path_to_read: modpack_config_path.display().to_string(),
                    error_message: error.to_string(),
                });
            }
        };

        let mut modpack: Modpack = match serde_json::from_str(&config) {
            Ok(modpack) => modpack,
            Err(error) => {
                return Err(PackrinthError::FailedToParseConfigJson {
                    config_path: modpack_config_path.display().to_string(),
                    error_message: error.to_string(),
                });
            }
        };

        modpack.directory = PathBuf::from(directory);
        modpack.modpack_config_path = modpack_config_path;

        Ok(modpack)
    }

    /// Adds projects to the modpack with optional version overrides or inclusions or exclusions.
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

    /// Adds a version override to a project added to this modpack.
    ///
    /// # Errors
    /// - [`PackrinthError::ProjectIsNotAdded`] if given project isn't added to the modpack
    pub fn add_version_override(
        &mut self,
        project: &str,
        branch: &str,
        project_version_id: &str,
    ) -> Result<(), PackrinthError> {
        let Some(project_settings) = self.projects.get_mut(project) else {
            return Err(PackrinthError::ProjectIsNotAdded {
                project: project.to_string(),
            });
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

    /// Removes a version override from a project added to this modpack.
    ///
    /// # Errors
    /// - [`PackrinthError::ProjectIsNotAdded`] if given project isn't added to the modpack
    /// - [`PackrinthError::OverrideDoesNotExist`] if given override doesn't exist for given project
    /// - [`PackrinthError::NoOverridesForProject`] if no overrides exist for given project at all
    pub fn remove_version_override(
        &mut self,
        project: &str,
        branch: &str,
    ) -> Result<(), PackrinthError> {
        let Some(project_settings) = self.projects.get_mut(project) else {
            return Err(PackrinthError::ProjectIsNotAdded {
                project: project.to_string(),
            });
        };

        if let Some(version_overrides) = &mut project_settings.version_overrides {
            // shift_remove to show Git that one line was removed
            if version_overrides.shift_remove(branch).is_none() {
                Err(PackrinthError::OverrideDoesNotExist {
                    project: project.to_string(),
                    branch: branch.to_string(),
                })
            } else {
                Ok(())
            }
        } else {
            Err(PackrinthError::NoOverridesForProject {
                project: project.to_string(),
            })
        }
    }

    /// Removes all version overrides from a project.
    ///
    /// # Errors
    /// - [`PackrinthError::ProjectIsNotAdded`] if given project isn't added to the modpack
    pub fn remove_all_version_overrides(&mut self, project: &str) -> Result<(), PackrinthError> {
        if let Some(project_settings) = self.projects.get_mut(project) {
            project_settings.version_overrides = None;
            Ok(())
        } else {
            Err(PackrinthError::ProjectIsNotAdded {
                project: project.to_string(),
            })
        }
    }

    /// Adds inclusions to a project.
    ///
    /// # Errors
    /// - [`PackrinthError::ProjectIsNotAdded`] if given project isn't added to the modpack
    /// - [`PackrinthError::ProjectAlreadyHasExclusions`] if the project already has exclusions
    pub fn add_project_inclusions(
        &mut self,
        project: &str,
        new_inclusions: &[String],
    ) -> Result<(), PackrinthError> {
        let Some(project_settings) = self.projects.get_mut(project) else {
            return Err(PackrinthError::ProjectIsNotAdded {
                project: project.to_string(),
            });
        };

        if let Some(include_or_exclude) = &mut project_settings.include_or_exclude {
            if let IncludeOrExclude::Include(inclusions) = include_or_exclude {
                for new_include in new_inclusions {
                    inclusions.push(new_include.clone());
                }
            } else {
                return Err(PackrinthError::ProjectAlreadyHasExclusions {
                    project: project.to_string(),
                });
            }
        } else {
            project_settings.include_or_exclude =
                Some(IncludeOrExclude::Include(Vec::from(new_inclusions)));
        }

        Ok(())
    }

    /// Removes inclusions from a project.
    ///
    /// # Errors
    /// - [`PackrinthError::ProjectIsNotAdded`] if given project isn't added to the modpack
    /// - [`PackrinthError::NoInclusionsForProject`] if no inclusions exist for the project
    pub fn remove_project_inclusions(
        &mut self,
        project: &str,
        inclusions_to_remove: &[String],
    ) -> Result<(), PackrinthError> {
        let Some(project_settings) = self.projects.get_mut(project) else {
            return Err(PackrinthError::ProjectIsNotAdded {
                project: project.to_string(),
            });
        };

        if let Some(include_or_exclude) = &mut project_settings.include_or_exclude
            && let IncludeOrExclude::Include(inclusions) = include_or_exclude
        {
            inclusions.retain(|x| !inclusions_to_remove.contains(x));
            Ok(())
        } else {
            Err(PackrinthError::NoInclusionsForProject {
                project: project.to_string(),
            })
        }
    }

    /// Removes all project inclusions.
    ///
    /// # Errors
    /// - [`PackrinthError::ProjectIsNotAdded`] if given project isn't added to the modpack
    /// - [`PackrinthError::NoInclusionsForProject`] if no inclusions exist for the project
    pub fn remove_all_project_inclusions(&mut self, project: &str) -> Result<(), PackrinthError> {
        if let Some(project_settings) = self.projects.get_mut(project)
            && let Some(include_or_exclude) = &project_settings.include_or_exclude
        {
            // Safety check to see if the user accidentally typed include instead of exclude
            if let IncludeOrExclude::Include(_) = include_or_exclude {
                project_settings.include_or_exclude = None;
                Ok(())
            } else {
                Err(PackrinthError::NoInclusionsForProject {
                    project: project.to_string(),
                })
            }
        } else {
            Err(PackrinthError::ProjectIsNotAdded {
                project: project.to_string(),
            })
        }
    }

    /// Adds exclusions to a project.
    ///
    /// # Errors
    /// - [`PackrinthError::ProjectIsNotAdded`] if given project isn't added to the modpack
    /// - [`PackrinthError::ProjectAlreadyHasInclusions`] if the project already has inclusions
    pub fn add_project_exclusions(
        &mut self,
        project: &str,
        new_exclusions: &[String],
    ) -> Result<(), PackrinthError> {
        let Some(project_settings) = self.projects.get_mut(project) else {
            return Err(PackrinthError::ProjectIsNotAdded {
                project: project.to_string(),
            });
        };

        if let Some(include_or_exclude) = &mut project_settings.include_or_exclude {
            if let IncludeOrExclude::Exclude(exclusions) = include_or_exclude {
                for new_exclude in new_exclusions {
                    exclusions.push(new_exclude.clone());
                }
            } else {
                return Err(PackrinthError::ProjectAlreadyHasInclusions {
                    project: project.to_string(),
                });
            }
        } else {
            project_settings.include_or_exclude =
                Some(IncludeOrExclude::Exclude(Vec::from(new_exclusions)));
        }

        Ok(())
    }

    /// Removes exclusions from a project.
    ///
    /// # Errors
    /// - [`PackrinthError::ProjectIsNotAdded`] if given project isn't added to the modpack
    /// - [`PackrinthError::NoExclusionsForProject`] if no exclusions exist for the project
    pub fn remove_project_exclusions(
        &mut self,
        project: &str,
        exclusions_to_remove: &[String],
    ) -> Result<(), PackrinthError> {
        let Some(project_settings) = self.projects.get_mut(project) else {
            return Err(PackrinthError::ProjectIsNotAdded {
                project: project.to_string(),
            });
        };

        if let Some(include_or_exclude) = &mut project_settings.include_or_exclude
            && let IncludeOrExclude::Exclude(exclusions) = include_or_exclude
        {
            exclusions.retain(|x| !exclusions_to_remove.contains(x));
            Ok(())
        } else {
            Err(PackrinthError::NoExclusionsForProject {
                project: project.to_string(),
            })
        }
    }

    /// Removes all exclusions from a project.
    ///
    /// # Errors
    /// - [`PackrinthError::ProjectIsNotAdded`] if given project isn't added to the modpack
    /// - [`PackrinthError::NoExclusionsForProject`] if no exclusions exist for the project
    pub fn remove_all_project_exclusions(&mut self, project: &str) -> Result<(), PackrinthError> {
        if let Some(project_settings) = self.projects.get_mut(project)
            && let Some(include_or_exclude) = &project_settings.include_or_exclude
        {
            // Safety check to see if the user accidentally typed exclude instead of include
            if let IncludeOrExclude::Exclude(_) = include_or_exclude {
                project_settings.include_or_exclude = None;
                Ok(())
            } else {
                Err(PackrinthError::NoExclusionsForProject {
                    project: project.to_string(),
                })
            }
        } else {
            Err(PackrinthError::ProjectIsNotAdded {
                project: project.to_string(),
            })
        }
    }

    /// Removes projects from the modpack.
    pub fn remove_projects(&mut self, projects: &[String]) {
        for project in projects {
            // shift_remove to show Git that one line was removed
            self.projects.shift_remove(&String::from(project));
        }
    }

    /// Creates new branches.
    ///
    /// This function also creates the branch directory.
    /// If the branch already exists, it just returns the existing branch.
    ///
    /// # Errors
    /// - [`PackrinthError::FailedToCreateDir`] if the creation of the branch directory failed
    pub fn new_branch(&mut self, name: &String) -> Result<BranchConfig, PackrinthError> {
        if !self.branches.contains(name) {
            self.branches.push(name.clone());
        }
        let branch_dir = self.directory.join(name);
        if let Ok(exists) = fs::exists(&branch_dir)
            && !exists
            && let Err(error) = fs::create_dir(&branch_dir)
        {
            return Err(PackrinthError::FailedToCreateDir {
                dir_to_create: branch_dir.display().to_string(),
                error_message: error.to_string(),
            });
        }
        BranchConfig::from_directory(&self.directory, name)
    }

    /// Removes branches from the modpack.
    ///
    /// It also removes the branch directories. If something goes wrong during this process,
    /// all errors get ignored.
    ///
    /// # Errors
    /// - [`PackrinthError::FailedToRemoveDir`] if removing the branch directory failed
    pub fn remove_branches(&mut self, branch_names: &Vec<String>) -> Result<(), PackrinthError> {
        for branch_name in branch_names {
            let branch_path = self.directory.join(branch_name);

            if self.branches.contains(branch_name) {
                self.branches.retain(|x| x != branch_name);
                if let Ok(exists) = fs::exists(&branch_path)
                    && exists
                    && let Err(error) = fs::remove_dir_all(&branch_path)
                {
                    return Err(PackrinthError::FailedToRemoveDir {
                        dir_to_remove: branch_path.display().to_string(),
                        error_message: error.to_string(),
                    });
                }
            }
        }

        Ok(())
    }

    /// Saves the modpack to the configuration file.
    ///
    /// # Errors
    /// - [`PackrinthError::FailedToSerialize`] if serialising this type to a JSON failed
    /// - [`PackrinthError::FailedToWriteFile`] if writing the JSON to a file failed
    pub fn save(&self) -> Result<(), PackrinthError> {
        json_to_file(self, &self.modpack_config_path)
    }

    /// Exports a branch to a `.mrpack` file.
    ///
    /// The path of the exported modpack will be a file in the branch directory.
    /// The full path gets returned if exporting was successful.
    ///
    /// # Errors
    /// - [`PackrinthError::FailedToSerialize`] if serializing the main manifest goes wrong
    ///
    /// Other errors may occur while creating the zip file:
    /// - [`PackrinthError::FailedToInitializeFileType`]
    /// - [`PackrinthError::FailedToStartZipFile`]
    /// - [`PackrinthError::FailedToWriteToZip`]
    /// - [`PackrinthError::FailedToGetWalkDirEntry`]
    /// - [`PackrinthError::FailedToStripPath`]
    /// - [`PackrinthError::FailedToCopyIntoBuffer`]
    /// - [`PackrinthError::FailedToAddZipDir`]
    /// - [`PackrinthError::FailedToFinishZip`]
    // Allow because it's hard to split this function up in other functions
    // without them having lots of parameters.
    #[allow(clippy::too_many_lines)]
    pub fn export_branch(&self, branch: &String) -> Result<PathBuf, PackrinthError> {
        let branch_config = BranchConfig::from_directory(&self.directory, branch)?;
        let branch_files = BranchFiles::from_directory(&self.directory, branch)?;

        let mrpack_file_name = format!("{}_{}.mrpack", self.name, branch_config.version);
        let branch_dir = self.directory.join(branch);
        let target_dir = self.directory.join(TARGET_DIRECTORY).join(branch);
        if let Err(error) = fs::create_dir_all(&target_dir) {
            return Err(PackrinthError::FailedToCreateDir {
                dir_to_create: target_dir.display().to_string(),
                error_message: error.to_string(),
            });
        }
        let mrpack_path = target_dir.join(&mrpack_file_name);

        let mrpack = MrPack {
            format_version: MODRINTH_PACK_FORMAT,
            game: GAME.to_string(),
            version_id: branch_config.version.clone(),
            name: self.name.clone(),
            summary: Some(self.summary.clone()),
            files: branch_files.files,
            dependencies: Self::create_dependencies(branch_config)?,
        };

        let mrpack_json = match serde_json_to_string_pretty(&mrpack) {
            Ok(mrpack_json) => mrpack_json,
            Err(error) => {
                return Err(PackrinthError::FailedToSerialize {
                    error_message: error.to_string(),
                });
            }
        };
        let options = SimpleFileOptions::default();
        let zip_file = match fs::File::create(&mrpack_path) {
            Ok(zip_file) => zip_file,
            Err(error) => {
                return Err(PackrinthError::FailedToInitializeFileType {
                    file_to_create: mrpack_path.display().to_string(),
                    error_message: error.to_string(),
                });
            }
        };

        let mut zip = ZipWriter::new(zip_file);
        if let Err(error) = zip.start_file(MRPACK_CONFIG_FILE_NAME, options) {
            return Err(PackrinthError::FailedToStartZipFile {
                file_to_start: MRPACK_CONFIG_FILE_NAME.to_string(),
                error_message: error.to_string(),
            });
        }
        if let Err(error) = zip.write_all(mrpack_json.as_bytes()) {
            return Err(PackrinthError::FailedToWriteToZip {
                to_write: MRPACK_CONFIG_FILE_NAME.to_string(),
                error_message: error.to_string(),
            });
        }

        // If some items are skipped in the loop, this is set to Err, and it will be returned at the end.
        let mut result = Ok(());

        // Loop every file/dir in the override dirs
        for override_dir in OVERRIDE_DIRS {
            let override_dir_path = branch_dir.join(override_dir);

            // Skip override dir if it doesn't exist.
            if let Ok(exists) = fs::exists(&override_dir_path)
                && !exists
            {
                continue;
            }

            for entry in WalkDir::new(override_dir_path) {
                let entry = match entry {
                    Ok(entry) => entry,
                    Err(error) => {
                        result = Err(PackrinthError::FailedToGetWalkDirEntry {
                            error_message: error.to_string(),
                        });
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
                    result = Err(PackrinthError::FailedToStripPath {
                        path: path.display().to_string(),
                    });
                    continue;
                };

                if path.is_file() {
                    if let Err(error) = zip.start_file(zip_path, options) {
                        result = Err(PackrinthError::FailedToStartZipFile {
                            file_to_start: zip_path.to_string(),
                            error_message: error.to_string(),
                        });
                        continue;
                    }
                    let mut buffer = Vec::new();
                    let mut original_file = match fs::File::open(path) {
                        Ok(file) => file,
                        Err(error) => {
                            result = Err(PackrinthError::FailedToInitializeFileType {
                                file_to_create: path.display().to_string(),
                                error_message: error.to_string(),
                            });
                            continue;
                        }
                    };
                    if let Err(_error) = io::copy(&mut original_file, &mut buffer) {
                        result = Err(PackrinthError::FailedToCopyIntoBuffer);
                        continue;
                    }
                    if let Err(error) = zip.write_all(&buffer) {
                        result = Err(PackrinthError::FailedToWriteToZip {
                            to_write: String::from_utf8_lossy(&buffer).to_string(),
                            error_message: error.to_string(),
                        });
                    }
                } else if path.is_dir()
                    && let Err(_error) = zip.add_directory(zip_path, options)
                {
                    result = Err(PackrinthError::FailedToAddZipDir {
                        zip_dir_path: zip_path.to_string(),
                    });
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

    fn create_dependencies(
        branch_config: BranchConfig,
    ) -> Result<MrPackDependencies, PackrinthError> {
        let mut forge = None;
        let mut neoforge = None;
        let mut fabric_loader = None;
        let mut quilt_loader = None;

        if let Some(main_loader) = branch_config.mod_loader {
            let Some(loader_version) = branch_config.loader_version else {
                return Err(PackrinthError::MainModLoaderProvidedButNoVersion);
            };

            match main_loader {
                MainLoader::Forge => forge = Some(loader_version),
                MainLoader::NeoForge => neoforge = Some(loader_version),
                MainLoader::Fabric => fabric_loader = Some(loader_version),
                MainLoader::Quilt => quilt_loader = Some(loader_version),
            }
        }

        Ok(MrPackDependencies {
            minecraft: branch_config.minecraft_version,
            forge,
            neoforge,
            fabric_loader,
            quilt_loader,
        })
    }
}

impl Default for Modpack {
    fn default() -> Self {
        Self {
            pack_format: CURRENT_PACK_FORMAT,
            name: "My Modrinth modpack".to_string(),
            summary: "Short summary for this modpack".to_string(),
            author: "John Doe".to_string(),
            require_all: false,
            auto_dependencies: true,
            branches: Vec::default(),
            projects: IndexMap::default(),
            directory: PathBuf::default(),
            modpack_config_path: PathBuf::default(),
        }
    }
}

impl BranchConfig {
    /// Gets a branch configuration type from a directory and name.
    ///
    /// # Errors
    /// - [`PackrinthError::FailedToParseConfigJson`] if the branch configuration file was invalid
    /// - [`PackrinthError::FailedToReadToString`] if reading the configuration file failed
    /// - [`PackrinthError::DirectoryExpected`] if the given directory is not a directory
    /// - [`PackrinthError::BranchDoesNotExist`] if the branch doesn't exist
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
                                    return Err(PackrinthError::FailedToParseConfigJson {
                                        config_path: branch_config_path.display().to_string(),
                                        error_message: error.to_string(),
                                    });
                                }
                            };
                            branch_config
                        }
                        Err(error) => {
                            if error.kind() == io::ErrorKind::NotFound {
                                let default_branch_config = Self::default();
                                default_branch_config.save(directory, name)?;
                                default_branch_config
                            } else {
                                return Err(PackrinthError::FailedToReadToString {
                                    path_to_read: branch_config_path.display().to_string(),
                                    error_message: error.to_string(),
                                });
                            }
                        }
                    };
                    Ok(branch_config)
                } else {
                    Err(PackrinthError::DirectoryExpected {
                        path_that_should_have_been_dir: branch_dir.display().to_string(),
                    })
                }
            }
            Err(error) => Err(PackrinthError::BranchDoesNotExist {
                branch: name.clone(),
                error_message: error.to_string(),
            }),
        }
    }

    /// Saves the branch configuration to the directory and name of the branch.
    ///
    /// # Errors
    /// - [`PackrinthError::FailedToSerialize`] if serialising this type to a JSON failed
    /// - [`PackrinthError::FailedToWriteFile`] if writing the JSON to a file failed
    pub fn save(&self, directory: &Path, name: &String) -> Result<(), PackrinthError> {
        let branch_config_path = directory.join(name).join(BRANCH_CONFIG_FILE_NAME);
        json_to_file(self, branch_config_path)
    }

    /// Prints a representation of the branch.
    ///
    /// Example:
    /// ```text
    /// Branch 1.20.1:
    ///   - Branch version: 1.0.0+1.20.1-alpha.1
    ///   - Main Minecraft version: 1.20.1
    ///   - Acceptable Minecraft versions:
    ///   - Main mod loader: Fabric
    ///   - Main mod loader version: 0.17.2
    ///   - Acceptable loaders: Iris
    ///   - No manual files are added
    /// ```
    ///
    /// # Errors
    /// - [`PackrinthError::MainModLoaderProvidedButNoVersion`] if a main mod loader was specified
    ///   in the configuration file, but no version for the mod loader was set
    pub fn print_display(&self, name: &str) -> Result<(), PackrinthError> {
        println!("Branch {name}:");
        println!("  - Branch version: {}", self.version);
        println!("  - Main Minecraft version: {}", self.minecraft_version);
        println!(
            "  - Acceptable Minecraft versions: {}",
            self.acceptable_minecraft_versions.join(", ")
        );
        if let Some(mod_loader) = &self.mod_loader {
            println!("  - Main mod loader: {}", mod_loader.pretty_value());
            match &self.loader_version {
                None => return Err(PackrinthError::MainModLoaderProvidedButNoVersion),
                Some(loader_version) => println!("  - Main mod loader version: {loader_version}"),
            }
        }
        println!(
            "  - Acceptable loaders: {}",
            Loader::pretty_value_vec(&self.acceptable_loaders).join(", ")
        );
        if self.manual_files.is_empty() {
            println!("  - No manual files are added");
        } else {
            println!("  - Has manual files added, see the configuration file");
        }

        Ok(())
    }
}

impl Default for BranchConfig {
    fn default() -> Self {
        Self {
            version: "1.0.0-fabric".to_string(),
            minecraft_version: "1.21.8".to_string(),
            acceptable_minecraft_versions: vec!["1.21.6".to_string(), "1.21.7".to_string()],
            mod_loader: Some(MainLoader::Fabric),
            loader_version: Some("0.17.2".to_string()),
            acceptable_loaders: vec![Loader::Minecraft, Loader::VanillaShader],
            manual_files: vec![],
        }
    }
}

impl BranchFiles {
    /// Gets a branch files type from a branch directory and name.
    ///
    /// # Errors
    /// - [`PackrinthError::FailedToParseConfigJson`] if the branch files configuration file was invalid
    /// - [`PackrinthError::FailedToReadToString`] if reading the configuration file failed
    /// - [`PackrinthError::DirectoryExpected`] if the given directory is not a directory
    /// - [`PackrinthError::BranchDoesNotExist`] if the branch doesn't exist
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
                                    return Err(PackrinthError::FailedToParseConfigJson {
                                        config_path: branch_files_path.display().to_string(),
                                        error_message: error.to_string(),
                                    });
                                }
                            };
                            branch_files
                        }
                        Err(error) => {
                            if error.kind() == io::ErrorKind::NotFound {
                                let default_branch_files = Self::default();
                                default_branch_files.save(directory, name)?;
                                default_branch_files
                            } else {
                                return Err(PackrinthError::FailedToReadToString {
                                    path_to_read: branch_files_path.display().to_string(),
                                    error_message: error.to_string(),
                                });
                            }
                        }
                    };
                    Ok(Self {
                        info: BRANCH_FILES_INFO.to_string(),
                        projects: branch_files.projects,
                        files: branch_files.files,
                    })
                } else {
                    Err(PackrinthError::DirectoryExpected {
                        path_that_should_have_been_dir: branch_dir.display().to_string(),
                    })
                }
            }
            Err(error) => Err(PackrinthError::BranchDoesNotExist {
                branch: name.clone(),
                error_message: error.to_string(),
            }),
        }
    }

    /// Saves the current files configuration to the directory and name of the branch.
    ///
    /// # Errors
    /// - [`PackrinthError::FailedToSerialize`] if serialising this type to a JSON failed
    /// - [`PackrinthError::FailedToWriteFile`] if writing the JSON to a file failed
    pub fn save(&self, directory: &Path, name: &String) -> Result<(), PackrinthError> {
        let branch_files_path = directory.join(name).join(BRANCH_FILES_FILE_NAME);
        json_to_file(self, branch_files_path)
    }
}

impl Default for BranchFiles {
    fn default() -> Self {
        Self {
            info: BRANCH_FILES_INFO.to_string(),
            projects: vec![],
            files: vec![],
        }
    }
}

impl MainLoader {
    /// Returns a pretty human-friendly value.
    #[must_use]
    pub const fn pretty_value(&self) -> &str {
        match self {
            MainLoader::Forge => "Forge",
            MainLoader::NeoForge => "NeoForge",
            MainLoader::Fabric => "Fabric",
            MainLoader::Quilt => "Quilt",
        }
    }

    /// Returns the Modrinth value, which can be used in the URL of web requests.
    #[must_use]
    pub const fn modrinth_value(&self) -> &str {
        match self {
            MainLoader::Forge => "forge",
            MainLoader::NeoForge => "neoforge",
            MainLoader::Fabric => "fabric",
            MainLoader::Quilt => "quilt",
        }
    }
}

impl Loader {
    /// Returns a pretty human-friendly value [`Vec`].
    #[must_use]
    pub fn pretty_value_vec(loaders: &Vec<Self>) -> Vec<&str> {
        let mut values = Vec::new();
        for loader in loaders {
            values.push(loader.pretty_value());
        }
        values
    }

    /// Returns the Modrinth value [`Vec`], which can be used in the URL of web requests.
    #[must_use]
    pub fn modrinth_value_vec(loaders: &Vec<Self>) -> Vec<&str> {
        let mut values = Vec::new();
        for loader in loaders {
            values.push(loader.modrinth_value());
        }
        values
    }

    /// Returns a pretty human-friendly value.
    #[must_use]
    pub const fn pretty_value(&self) -> &str {
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

    /// Returns the Modrinth value, which can be used in the URL of web requests.
    #[must_use]
    pub const fn modrinth_value(&self) -> &str {
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
