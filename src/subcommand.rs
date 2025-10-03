use crate::{Cli, print_error, print_success, single_line_error};
use clap::CommandFactory;
use clap_complete::{Generator, shells};
use dialoguer::Confirm;
use indexmap::IndexMap;
use packrinth::config::{
    BranchConfig, BranchFiles, BranchFilesProject, IncludeOrExclude, MainLoader, Modpack,
    ProjectSettings,
};
use packrinth::modrinth::{
    MrPack, Project, Version, VersionDependency, VersionDependencyType, extract_mrpack,
};
use packrinth::{
    GitUtils, PackrinthError, ProjectMarkdownTable, ProjectUpdateResult, ProjectUpdater, config,
};
use progress_bar::pb::ProgressBar;
use progress_bar::{Color, Style};
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use std::path::Path;
use std::{cmp, fs, io};

// Allow because we need all of them
#[allow(clippy::wildcard_imports)]
use crate::cli::*;

fn create_progress_bar(max: usize) -> ProgressBar {
    let mut progress_bar = ProgressBar::new_with_eta(max);
    progress_bar.set_progress_style(progress_bar::ProgressStyle::Percentage);
    if let Some((terminal_size::Width(width), terminal_size::Height(_height))) =
        terminal_size::terminal_size()
    {
        // Decrease width when the terminal is small. Otherwise, take 50 columns.
        // Subtracting results in u16, which is safe to convert to usize.
        #[allow(clippy::as_conversions)]
        progress_bar.set_width(cmp::min(width.saturating_sub(45) as usize, 50));
    }
    progress_bar
}

impl Cli {
    pub fn run(&mut self) {
        if let Err(error) = self.subcommand.run(&self.config_args) {
            print_error(error.message_and_tip());
        }
    }
}

impl SubCommand {
    fn run(&self, config_args: &ConfigArgs) -> Result<(), PackrinthError> {
        if let SubCommand::Version(args) = self {
            return args.run(config_args);
        } else if let SubCommand::Completions(args) = self {
            return args.run(config_args);
        }

        let mut current_dir = match config_args.directory.clone() {
            Some(dir) => dir,
            None => match std::env::current_dir() {
                Ok(current_dir) => current_dir.clone(),
                Err(error) => {
                    return Err(PackrinthError::FailedToGetCurrentDirectory {
                        error_message: error.to_string(),
                    });
                }
            },
        };

        if let Self::Init(args) = self {
            return args.run(&current_dir, config_args);
        }

        let mut modpack = loop {
            match Modpack::from_directory(&current_dir) {
                Ok(modpack) => break modpack,
                Err(error) => {
                    if let PackrinthError::FailedToReadToString { path_to_read, .. } = &error
                        && path_to_read.contains(config::MODPACK_CONFIG_FILE_NAME)
                        && let Some(parent) = current_dir.parent()
                    {
                        current_dir = parent.to_path_buf();
                    } else {
                        return Err(error);
                    }
                }
            }
        };

        if modpack.pack_format != config::CURRENT_PACK_FORMAT {
            return Err(PackrinthError::InvalidPackFormat {
                used_pack_format: modpack.pack_format,
            });
        }

        match self {
            SubCommand::Import(args) => args.run(&mut modpack, config_args),
            SubCommand::Project(args) => args.run(&mut modpack, config_args),
            SubCommand::Branch(args) => args.run(&mut modpack, config_args),
            SubCommand::Update(args) => args.run(&modpack, config_args),
            SubCommand::Export(args) => args.run(&modpack, config_args),
            SubCommand::Clean(args) => args.run(&modpack, config_args),
            SubCommand::Doc(args) => args.run(&modpack, config_args),
            _ => Ok(()), // These cases should have been handled before this match statement.
        }
    }
}

impl InitArgs {
    pub fn run(&self, directory: &Path, _config_args: &ConfigArgs) -> Result<(), PackrinthError> {
        let directory = if let Some(modpack_name) = &self.modpack_name {
            &directory.join(modpack_name)
        } else {
            directory
        };

        let mut modpack = Modpack::new(directory, self.force)?;
        if let Some(modpack_name) = &self.modpack_name {
            modpack.name.clone_from(modpack_name);
        }
        modpack.save()?;

        if !self.no_git_repo {
            GitUtils::initialize_modpack_repo(directory)?;
        }

        print_success(format!(
            "created new modpack in directory {}",
            directory.display()
        ));
        Ok(())
    }
}

impl ImportArgs {
    // TODO move into lib
    pub fn run(
        &self,
        modpack: &mut Modpack,
        _config_args: &ConfigArgs,
    ) -> Result<(), PackrinthError> {
        if self.add_projects && !self.allow_dirty && GitUtils::modpack_is_dirty(modpack) {
            return Err(PackrinthError::RepoIsDirty);
        }

        let mrpack = MrPack::from_mrpack(&self.modrinth_pack)?;

        let branch_name = match &self.modrinth_pack.file_name() {
            Some(branch_name) => branch_name.display().to_string(),
            None => self.modrinth_pack.display().to_string(),
        }
        .split(".mrpack")
        .collect::<Vec<&str>>()[0]
            .to_string();

        // Check if branch already exists
        if modpack.branches.contains(&branch_name) && !self.force {
            return Err(PackrinthError::BranchAlreadyExists {
                branch: branch_name,
            });
        }

        let mut branch_config = modpack.new_branch(&branch_name)?;
        branch_config.version.clone_from(&branch_name);
        branch_config.minecraft_version = mrpack.dependencies.minecraft;
        branch_config.acceptable_minecraft_versions = Vec::new();
        if let Some(loader_version) = mrpack.dependencies.fabric_loader {
            branch_config.mod_loader = Some(MainLoader::Fabric);
            branch_config.loader_version = Some(loader_version);
        } else if let Some(loader_version) = mrpack.dependencies.forge {
            branch_config.mod_loader = Some(MainLoader::Forge);
            branch_config.loader_version = Some(loader_version);
        } else if let Some(loader_version) = mrpack.dependencies.neoforge {
            branch_config.mod_loader = Some(MainLoader::NeoForge);
            branch_config.loader_version = Some(loader_version);
        } else if let Some(loader_version) = mrpack.dependencies.quilt_loader {
            branch_config.mod_loader = Some(MainLoader::Quilt);
            branch_config.loader_version = Some(loader_version);
        }
        branch_config.save(&modpack.directory, &branch_name)?;

        let mut branch_files = BranchFiles::from_directory(&modpack.directory, &branch_name)?;
        branch_files.files.clone_from(&mrpack.files);

        let mut progress_bar = create_progress_bar(mrpack.files.len());
        progress_bar.set_action("importing", Color::Blue, Style::Bold);

        for file in mrpack.files {
            let version = match Version::from_sha512_hash(&file.hashes.sha512) {
                Ok(version) => version,
                Err(_error) => continue,
            };
            let project = Project::from_id(&version.project_id)?;

            branch_files.projects.push(BranchFilesProject {
                name: project.title,
                id: Some(project.slug.clone()),
            });

            if self.add_projects && !modpack.projects.contains_key(&version.project_id)
                || !modpack.projects.contains_key(&project.slug)
            {
                modpack.projects.insert(
                    project.slug,
                    ProjectSettings {
                        version_overrides: None,
                        include_or_exclude: None,
                    },
                );
            }

            progress_bar.inc();
        }

        branch_files.save(&modpack.directory, &branch_name)?;
        modpack.save()?;

        let mrpack_output = &modpack.directory.join(&branch_name);
        if let Err(error) = extract_mrpack(&self.modrinth_pack, mrpack_output) {
            return Err(PackrinthError::FailedToExtractMrPack {
                mrpack_path: self.modrinth_pack.display().to_string(),
                output_directory: mrpack_output.display().to_string(),
                error_message: error.to_string(),
            });
        }

        progress_bar.print_info(
            "success",
            &format!("imported {}", &self.modrinth_pack.display()),
            Color::Green,
            Style::Bold,
        );
        Ok(())
    }
}

impl ProjectArgs {
    pub fn run(
        &self,
        modpack: &mut Modpack,
        config_args: &ConfigArgs,
    ) -> Result<(), PackrinthError> {
        if let Some(command) = &self.command {
            match command {
                ProjectSubCommand::List(args) => args.run(modpack, config_args),
                ProjectSubCommand::Add(args) => args.run(modpack, config_args),
                ProjectSubCommand::VersionOverride(args) => args.run(modpack, config_args),
                ProjectSubCommand::Inclusions(args) => args.run(modpack, config_args),
                ProjectSubCommand::Exclusions(args) => args.run(modpack, config_args),
                ProjectSubCommand::Remove(args) => args.run(modpack, config_args),
            }
        } else if let Some(project_names) = &self.projects {
            modpack
                .projects
                .retain(|key, _| project_names.contains(key));
            ListProjectsArgs::list(&modpack.projects)
        } else {
            ListProjectsArgs::run(&ListProjectsArgs {}, modpack, config_args)
        }
    }
}

impl ListProjectsArgs {
    // Allow unused self, because then it is clear to the maintainer that self is available for code expansion.
    #[allow(clippy::unused_self)]
    pub fn run(
        &self,
        modpack: &mut Modpack,
        _config_args: &ConfigArgs,
    ) -> Result<(), PackrinthError> {
        Self::list(&modpack.projects)
    }

    // Allowing unnecessary return because it makes it clear that using ? in the function is ok.
    #[allow(clippy::unnecessary_wraps)]
    pub fn list(projects: &IndexMap<String, ProjectSettings>) -> Result<(), PackrinthError> {
        if projects.is_empty() {
            println!("There are no projects added to this modpack yet.");
            return Ok(());
        }

        for project in projects {
            println!("{}", project.0);

            if let Some(overrides) = &project.1.version_overrides {
                println!("  - Overrides:");
                for version_override in overrides {
                    println!("    - {}: {}", version_override.0, version_override.1);
                }
            }

            if let Some(include_or_exclude) = &project.1.include_or_exclude {
                match include_or_exclude {
                    IncludeOrExclude::Include(inclusions) => {
                        println!("  - Inclusions: {}", inclusions.join(", "));
                    }
                    IncludeOrExclude::Exclude(exclusions) => {
                        println!("  - Exclusions: {}", exclusions.join(", "));
                    }
                }
            }
        }

        Ok(())
    }
}

impl AddProjectsArgs {
    pub fn run(
        &self,
        modpack: &mut Modpack,
        _config_args: &ConfigArgs,
    ) -> Result<(), PackrinthError> {
        let include_or_exclude = if let Some(include) = self.inclusions.clone() {
            Some(IncludeOrExclude::Include(include))
        } else {
            self.exclusions.clone().map(IncludeOrExclude::Exclude)
        };

        modpack.add_projects(&self.projects, &None, &include_or_exclude);
        modpack.save()?;

        print_success(format!("added {}", self.projects.join(", ")));
        Ok(())
    }
}

impl VersionOverrideProjectArgs {
    pub fn run(
        &self,
        modpack: &mut Modpack,
        config_args: &ConfigArgs,
    ) -> Result<(), PackrinthError> {
        match &self.command {
            VersionOverrideSubCommand::Add(args) => args.run(modpack, config_args),
            VersionOverrideSubCommand::Remove(args) => args.run(modpack, config_args),
        }
    }
}

impl AddVersionOverrideArgs {
    pub fn run(
        &self,
        modpack: &mut Modpack,
        _config_args: &ConfigArgs,
    ) -> Result<(), PackrinthError> {
        modpack.add_version_override(&self.project, &self.branch, &self.project_version_id)?;
        modpack.save()?;

        print_success(format!(
            "added version override for {}, branch {} and version ID {}",
            self.project, self.branch, self.project_version_id
        ));
        Ok(())
    }
}

impl RemoveVersionOverrideArgs {
    pub fn run(
        &self,
        modpack: &mut Modpack,
        _config_args: &ConfigArgs,
    ) -> Result<(), PackrinthError> {
        if self.all {
            modpack.remove_all_version_overrides(&self.project)?;
            modpack.save()?;

            print_success(format!("removed all overrides for {}", self.project));
            Ok(())
        } else if let Some(branch) = &self.branch {
            modpack.remove_version_override(branch, &self.project)?;
            modpack.save()?;

            print_success(format!("removed {} override for {}", self.project, branch));
            Ok(())
        } else {
            Err(PackrinthError::NoBranchSpecified)
        }
    }
}

impl InclusionsProjectArgs {
    pub fn run(
        &self,
        modpack: &mut Modpack,
        config_args: &ConfigArgs,
    ) -> Result<(), PackrinthError> {
        match &self.command {
            InclusionsSubCommand::Add(args) => args.run(modpack, config_args),
            InclusionsSubCommand::Remove(args) => args.run(modpack, config_args),
        }
    }
}

impl AddInclusionsArgs {
    pub fn run(
        &self,
        modpack: &mut Modpack,
        _config_args: &ConfigArgs,
    ) -> Result<(), PackrinthError> {
        modpack.add_project_inclusions(&self.project, &self.inclusions)?;
        modpack.save()?;

        print_success(format!(
            "added {} inclusions for {}",
            self.inclusions.join(", "),
            self.project
        ));
        Ok(())
    }
}

impl RemoveInclusionsArgs {
    pub fn run(
        &self,
        modpack: &mut Modpack,
        _config_args: &ConfigArgs,
    ) -> Result<(), PackrinthError> {
        if self.all {
            modpack.remove_all_project_inclusions(&self.project)?;
            modpack.save()?;

            print_success(format!("removed all inclusions for {}", self.project));
            Ok(())
        } else if let Some(inclusions) = &self.inclusions {
            modpack.remove_project_inclusions(&self.project, inclusions)?;
            modpack.save()?;

            print_success(format!(
                "removed {} inclusions for {}",
                inclusions.join(", "),
                self.project
            ));
            Ok(())
        } else {
            Err(PackrinthError::NoInclusionsSpecified)
        }
    }
}

impl ExclusionsProjectArgs {
    pub fn run(
        &self,
        modpack: &mut Modpack,
        config_args: &ConfigArgs,
    ) -> Result<(), PackrinthError> {
        match &self.command {
            ExclusionsSubCommand::Add(args) => args.run(modpack, config_args),
            ExclusionsSubCommand::Remove(args) => args.run(modpack, config_args),
        }
    }
}

impl AddExclusionsArgs {
    pub fn run(
        &self,
        modpack: &mut Modpack,
        _config_args: &ConfigArgs,
    ) -> Result<(), PackrinthError> {
        modpack.add_project_exclusions(&self.project, &self.exclusions)?;
        modpack.save()?;

        print_success(format!(
            "added {} exclusions for {}",
            self.exclusions.join(", "),
            self.project
        ));
        Ok(())
    }
}

impl RemoveExclusionsArgs {
    pub fn run(
        &self,
        modpack: &mut Modpack,
        _config_args: &ConfigArgs,
    ) -> Result<(), PackrinthError> {
        if self.all {
            modpack.remove_all_project_exclusions(&self.project)?;
            modpack.save()?;

            print_success(format!("removed all exclusions for {}", self.project));
            Ok(())
        } else if let Some(exclusions) = &self.exclusions {
            modpack.remove_project_exclusions(&self.project, exclusions)?;
            modpack.save()?;

            print_success(format!(
                "removed {} exclusions for {}",
                exclusions.join(", "),
                self.project
            ));
            Ok(())
        } else {
            Err(PackrinthError::NoExclusionsSpecified)
        }
    }
}

impl RemoveProjectsArgs {
    pub fn run(
        &self,
        modpack: &mut Modpack,
        _config_args: &ConfigArgs,
    ) -> Result<(), PackrinthError> {
        modpack.remove_projects(&self.projects);
        modpack.save()?;

        print_success(format!("removed {}", self.projects.join(", ")));
        Ok(())
    }
}
// TODO here
impl UpdateArgs {
    pub fn run(&self, modpack: &Modpack, config_args: &ConfigArgs) -> Result<(), PackrinthError> {
        if !self.allow_dirty && GitUtils::modpack_is_dirty(modpack) {
            return Err(PackrinthError::RepoIsDirty);
        }
        if modpack.branches.is_empty() {
            return Err(PackrinthError::ModpackHasNoBranchesToUpdate);
        }

        let branches = if let Some(branches) = &self.branches {
            branches
        } else {
            &modpack.branches
        };

        self.update_branches(
            modpack,
            branches,
            self.require_all || modpack.require_all,
            self.auto_dependencies || modpack.auto_dependencies,
            config_args.verbose,
        )
    }

    fn update_branches(
        &self,
        modpack: &Modpack,
        branches: &Vec<String>,
        require_all: bool,
        auto_dependencies: bool,
        verbose: bool,
    ) -> Result<(), PackrinthError> {
        let mut progress_bar = create_progress_bar(modpack.projects.len() * branches.len());

        for branch_name in branches {
            progress_bar.set_action(branch_name, Color::Blue, Style::Bold);

            let branch_config = BranchConfig::from_directory(&modpack.directory, branch_name)?;
            let mut branch_files =
                match BranchFiles::from_directory(&modpack.directory, branch_name) {
                    Ok(branch_files) => branch_files,
                    Err(_error) => {
                        let default_branch_files = BranchFiles::default();
                        default_branch_files.save(&modpack.directory, branch_name)?;
                        default_branch_files
                    }
                };

            // Remove all entries to ensure that there will be no duplicates if the user changes loaders
            branch_files.projects = Vec::new();
            branch_files.files = Vec::new();

            let mut dependencies: Vec<VersionDependency> = Vec::new();

            for (slug_project_id, project_settings) in &modpack.projects {
                let project_updater = ProjectUpdater {
                    branch_name,
                    branch_config: &branch_config,
                    branch_files: &mut branch_files,
                    slug_project_id,
                    project_settings,
                    require_all,
                    no_beta: self.no_beta,
                    no_alpha: self.no_alpha,
                };

                Self::update_project(
                    project_updater,
                    false,
                    &mut dependencies,
                    &mut progress_bar,
                    verbose,
                );

                progress_bar.inc();
            }

            if auto_dependencies {
                for dependency in dependencies {
                    if let Some(project_id) = dependency.project_id
                        && let VersionDependencyType::Required = dependency.dependency_type
                        && !branch_files
                            .projects
                            .iter()
                            .any(|project| project.id == Some(project_id.to_string()))
                    {
                        let project_settings = ProjectSettings {
                            version_overrides: None,
                            include_or_exclude: None,
                        };
                        let project_updater = ProjectUpdater {
                            branch_name,
                            branch_config: &branch_config,
                            branch_files: &mut branch_files,
                            slug_project_id: &project_id,
                            project_settings: &project_settings,
                            require_all,
                            no_beta: self.no_beta,
                            no_alpha: self.no_alpha,
                        };

                        // Create new vec because we don't care about the dependencies
                        Self::update_project(
                            project_updater,
                            true,
                            &mut Vec::new(),
                            &mut progress_bar,
                            verbose,
                        );
                    }
                }
            }

            // Copy manual files
            for manual_file in branch_config.manual_files {
                branch_files.projects.push(BranchFilesProject {
                    name: manual_file.project_name.clone(),
                    id: None,
                });
                branch_files.files.push(manual_file.clone());

                if verbose {
                    progress_bar.print_info(
                        "added",
                        &manual_file.project_name,
                        Color::Green,
                        Style::Normal,
                    );
                }
            }

            branch_files.save(&modpack.directory, branch_name)?;
        }

        progress_bar.print_final_info(
            "success:",
            &format!("updated {}", branches.join(", ")),
            Color::Green,
            Style::Bold,
        );

        Ok(())
    }

    fn update_project(
        mut project_updater: ProjectUpdater,
        is_dependency: bool,
        dependencies: &mut Vec<VersionDependency>,
        progress_bar: &mut ProgressBar,
        verbose: bool,
    ) {
        match project_updater.update_project() {
            ProjectUpdateResult::Added(new_dependencies) => {
                dependencies.extend(new_dependencies);

                let info_text = if is_dependency { "dependency" } else { "added" };

                if verbose {
                    progress_bar.print_info(
                        info_text,
                        project_updater.slug_project_id,
                        Color::Green,
                        Style::Normal,
                    );
                }
            }
            ProjectUpdateResult::Skipped => {
                if verbose {
                    progress_bar.print_info(
                        "skipped",
                        project_updater.slug_project_id,
                        Color::Yellow,
                        Style::Normal,
                    ); // TODO test if this displays correctly
                }
            }
            ProjectUpdateResult::NotFound => {
                if verbose {
                    progress_bar.print_info(
                        "not found",
                        project_updater.slug_project_id,
                        Color::Yellow,
                        Style::Bold,
                    );
                }
            }
            ProjectUpdateResult::Failed(error) => progress_bar.print_info(
                "failed",
                &single_line_error(error.message_and_tip()),
                Color::Red,
                Style::Bold,
            ),
        }
    }
}

impl BranchArgs {
    pub fn run(
        &self,
        modpack: &mut Modpack,
        config_args: &ConfigArgs,
    ) -> Result<(), PackrinthError> {
        if let Some(command) = &self.command {
            match command {
                BranchSubCommand::List(args) => args.run(modpack, config_args),
                BranchSubCommand::Add(args) => args.run(modpack, config_args),
                BranchSubCommand::Remove(args) => args.run(modpack, config_args),
            }
        } else if let Some(branch_names) = &self.branches {
            ListBranchesArgs::list(&modpack.directory, branch_names)
        } else {
            ListBranchesArgs::run(&ListBranchesArgs {}, modpack, config_args)
        }
    }
}

impl ListBranchesArgs {
    // Allow unused self, because then it is clear to the maintainer that self is available for code expansion.
    #[allow(clippy::unused_self)]
    pub fn run(&self, modpack: &Modpack, _config_args: &ConfigArgs) -> Result<(), PackrinthError> {
        Self::list(&modpack.directory, &modpack.branches)
    }

    pub fn list(directory: &Path, branches: &[String]) -> Result<(), PackrinthError> {
        if branches.is_empty() {
            println!("There are no branches added to this modpack yet.");
            return Ok(());
        }

        let mut iter = branches.iter().peekable();
        while let Some(branch_name) = iter.next() {
            match BranchConfig::from_directory(directory, branch_name) {
                Ok(branch) => {
                    if let Err(error) = branch.print_display(branch_name) {
                        print_error(error.message_and_tip());
                    }
                }
                Err(error) => {
                    if let PackrinthError::BranchDoesNotExist {
                        branch: _,
                        error_message: _,
                    } = error
                    {
                        println!(
                            "Branch {branch_name} is declared in the modpack config file, but it doesn't exist. Please consider removing it from the configuration or re-adding the branch.",
                        );
                    } else {
                        return Err(error);
                    }
                }
            }

            // Print new line between branches, but not at the very end.
            if iter.peek().is_some() {
                println!();
            }
        }

        Ok(())
    }
}

impl AddBranchesArgs {
    pub fn run(
        &self,
        modpack: &mut Modpack,
        _config_args: &ConfigArgs,
    ) -> Result<(), PackrinthError> {
        for branch_name in &self.branches {
            if let Err(error) = modpack.new_branch(branch_name) {
                // Don't use ? because then we can't try again for the next branch.
                print_error(error.message_and_tip());
            }
        }

        modpack.save()?;

        print_success(format!("added branches {}", self.branches.join(", ")));
        Ok(())
    }
}

impl RemoveBranchesArgs {
    // Allow because it is required in Cli::run.
    #[allow(clippy::unnecessary_wraps)]
    pub fn run(
        &self,
        modpack: &mut Modpack,
        _config_args: &ConfigArgs,
    ) -> Result<(), PackrinthError> {
        println!(
            "These branches in directory {} will be removed:",
            modpack.directory.display()
        );
        for branch in &self.branches {
            println!("  - {branch}");
        }
        println!(
            "Please keep in mind that all the content of the branches will be removed, including overrides."
        );
        println!();

        let confirmation = Confirm::new()
            .with_prompt("Do you want to continue?")
            .wait_for_newline(true)
            .default(false)
            .interact()
            .expect("Error while interacting with confirmation");
        println!();

        if confirmation {
            modpack.remove_branches(&self.branches)?;
            modpack.save()?;
            print_success(format!("removed {}", self.branches.join(", ")));
        } else {
            println!("Aborted action");
        }

        Ok(())
    }
}

impl ExportArgs {
    pub fn run(&self, modpack: &Modpack, _config_args: &ConfigArgs) -> Result<(), PackrinthError> {
        match &self.branches {
            None => Self::export_branches(modpack, &modpack.branches),
            Some(branches) => Self::export_branches(modpack, branches),
        }
    }

    // Allow because it is required in Cli::run.
    #[allow(clippy::unnecessary_wraps)]
    fn export_branches(modpack: &Modpack, branches: &Vec<String>) -> Result<(), PackrinthError> {
        for branch in branches {
            match modpack.export_branch(branch) {
                Ok(modpack_path) => {
                    print_success(format!("exported {} to {}", branch, modpack_path.display()));
                }
                Err(error) => {
                    // Don't use ? because then we can't try again for the next branch.
                    print_error(error.message_and_tip());
                }
            }
        }
        Ok(())
    }
}

impl CleanArgs {
    // Allow unused self, because then it is clear to the maintainer that self is available for code expansion.
    #[allow(clippy::unused_self)]
    pub fn run(&self, modpack: &Modpack, _config_args: &ConfigArgs) -> Result<(), PackrinthError> {
        let target_dir = modpack.directory.join(packrinth::TARGET_DIRECTORY);
        match fs::remove_dir_all(&target_dir) {
            Ok(()) => {
                print_success(format!("removed {}", target_dir.display()));
                Ok(())
            }
            Err(error) => Err(PackrinthError::FailedToRemoveDir {
                dir_to_remove: target_dir.display().to_string(),
                error_message: error.to_string(),
            }),
        }
    }
}

impl DocArgs {
    // Allow unused self, because then it is clear to the maintainer that self is available for code expansion.
    #[allow(clippy::unused_self)]
    pub fn run(&self, modpack: &Modpack, _config_args: &ConfigArgs) -> Result<(), PackrinthError> {
        let doc_markdown_table = modpack.generate_project_table()?;

        println!("# {} _by {}_", modpack.name, modpack.author);
        println!("{}", modpack.summary);

        if !doc_markdown_table.project_map.is_empty() {
            println!("## What is included?");
            println!("{doc_markdown_table}");
        }

        Ok(())
    }
}

impl CompletionsArgs {
    // Allow because it is required in Cli::run.
    #[allow(clippy::unnecessary_wraps)]
    pub fn run(&self, _config_args: &ConfigArgs) -> Result<(), PackrinthError> {
        let mut cmd = Cli::command();
        match self.shell {
            CompletionShell::Bash => Self::print_completions(shells::Bash, &mut cmd),
            CompletionShell::Elvish => Self::print_completions(shells::Elvish, &mut cmd),
            CompletionShell::Fish => Self::print_completions(shells::Fish, &mut cmd),
            CompletionShell::PowerShell => Self::print_completions(shells::PowerShell, &mut cmd),
            CompletionShell::Zsh => Self::print_completions(shells::Zsh, &mut cmd),
        }

        Ok(())
    }

    fn print_completions<G: Generator>(generator: G, cmd: &mut clap::Command) {
        clap_complete::generate(
            generator,
            cmd,
            cmd.get_name().to_string(),
            &mut io::stdout(),
        );
    }
}

impl VersionArgs {
    // Allow because it is required in Cli::run.
    #[allow(clippy::unnecessary_wraps)]
    // Allow unused self, because then it is clear to the maintainer that self is available for code expansion.
    #[allow(clippy::unused_self)]
    pub fn run(&self, _config_args: &ConfigArgs) -> Result<(), PackrinthError> {
        println!("Packrinth by {}", crate::AUTHORS);
        println!("Version {}", crate::VERSION);

        if let Ok(newest_version) = packrinth::is_new_version_available()
            && let Some(newest_version) = newest_version
        {
            println!(
                "A new version is available: {} v{}",
                env!("CARGO_PKG_NAME"),
                newest_version
            );
        }

        println!();
        println!(
            "If you find any bugs, have suggestions, or want to contribute, please visit the Git repository at:"
        );
        println!("{}", crate::REPOSITORY);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use tempdir::TempDir;

    fn init(test_modpack_dir: &Path) {
        Cli {
            subcommand: SubCommand::Init(InitArgs {
                modpack_name: None,
                no_git_repo: false,
                force: false,
            }),
            config_args: ConfigArgs {
                directory: Some(PathBuf::from(test_modpack_dir)),
                verbose: true,
            },
        }
        .run();

        // Check if .gitignore contains .mrpack
        assert!(
            fs::read_to_string(test_modpack_dir.join(".gitignore"))
                .unwrap()
                .contains("target")
        );
        // Check if modpack.json is right
        assert_eq!(
            "{
	\"pack_format\": 1,
	\"name\": \"My Modrinth modpack\",
	\"summary\": \"Short summary for this modpack\",
	\"author\": \"John Doe\",
	\"require_all\": false,
	\"auto_dependencies\": true,
	\"branches\": [],
	\"projects\": {}
}",
            fs::read_to_string(test_modpack_dir.join("modpack.json")).unwrap()
        );
    }

    #[test]
    fn test_projects() {
        let test_modpack_dir = TempDir::new("packrinth").unwrap().path().to_owned();
        init(&test_modpack_dir);

        // TODO test projects
    }
}
