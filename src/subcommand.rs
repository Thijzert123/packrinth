use crate::{Cli, print_error, print_success, single_line_error};
use clap::CommandFactory;
use clap_complete::{Generator, shells};
use dialoguer::Confirm;
use indexmap::IndexMap;
use packrinth::config::{
    BranchConfig, BranchFiles, BranchFilesProject, IncludeOrExclude, Modpack, ProjectSettings,
};
use packrinth::modrinth::{
    Env, File, FileResult, SideSupport, VersionDependency, VersionDependencyType,
};
use packrinth::{PackrinthError, config, modpack_is_dirty};
use progress_bar::pb::ProgressBar;
use progress_bar::{Color, Style};
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::{cmp, fs, io};

// Allow because we need all of them
#[allow(clippy::wildcard_imports)]
use crate::cli::*;

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
        }

        let current_dir = match &config_args.directory {
            Some(dir) => dir,
            None => match std::env::current_dir() {
                Ok(current_dir) => &current_dir.clone(),
                Err(error) => {
                    return Err(PackrinthError::FailedToGetCurrentDirectory {
                        error_message: error.to_string(),
                    });
                }
            },
        };

        if let Self::Init(args) = self {
            return args.run(current_dir, config_args);
        }

        let mut modpack = Modpack::from_directory(current_dir)?;

        if modpack.pack_format != config::CURRENT_PACK_FORMAT {
            return Err(PackrinthError::InvalidPackFormat {
                used_pack_format: modpack.pack_format,
            });
        }

        match self {
            SubCommand::Init(_args) => Ok(()),
            SubCommand::Version(_args) => Ok(()),
            SubCommand::Project(args) => args.run(&mut modpack, config_args),
            SubCommand::Branch(args) => args.run(&mut modpack, config_args),
            SubCommand::Update(args) => args.run(&modpack, config_args),
            SubCommand::Export(args) => args.run(&modpack, config_args),
            SubCommand::Doc(args) => args.run(&modpack, config_args),
            SubCommand::Completions(args) => args.run(&modpack, config_args),
        }
    }
}

impl InitArgs {
    pub fn run(&self, directory: &Path, _config_args: &ConfigArgs) -> Result<(), PackrinthError> {
        let modpack_config_path = directory.join(config::MODPACK_CONFIG_FILE_NAME);
        if !self.force
            && let Ok(exists) = fs::exists(&modpack_config_path)
            && exists
        {
            return Err(PackrinthError::ModpackAlreadyExists {
                directory: directory.display().to_string(),
            });
        }

        let modpack = Modpack::new(directory)?;

        modpack.save()?;

        if !self.no_git_repo
            && let Err(error) = gix::init(directory)
        {
            // If the repo already exists, don't show an error.
            if !matches!(
                &error,
                gix::init::Error::Init(gix::create::Error::DirectoryExists { path })
                    if path.file_name() == Some(std::ffi::OsStr::new(".git"))
            ) {
                return Err(PackrinthError::FailedToInitGitRepoWhileInitModpack {
                    error_message: error.to_string(),
                });
            }
        }

        let gitignore_path = directory.join(".gitignore");
        if let Ok(exists) = fs::exists(&gitignore_path)
            && !exists
            && let Ok(gitignore_file) = OpenOptions::new()
                .append(true)
                .create(true)
                .open(gitignore_path)
        {
            // If the gitignore file can't be written to, so be it.
            let _ = writeln!(&gitignore_file, "# Exported Modrinth modpacks");
            let _ = writeln!(&gitignore_file, "*.mrpack");
            let _ = gitignore_file.sync_all();
        }

        print_success(format!(
            "created new modpack in directory {}",
            directory.display()
        ));
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
        modpack.add_project_override(&self.project, &self.branch, &self.project_version_id)?;
        modpack.save()?;

        print_success(format!(
            "added override for {}, branch {} and version ID {}",
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
            modpack.remove_all_project_overrides(&self.project)?;
            modpack.save()?;

            print_success(format!("removed all overrides for {}", self.project));
            Ok(())
        } else if let Some(branch) = &self.branch {
            modpack.remove_project_override(branch, &self.project)?;
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

impl UpdateArgs {
    pub fn run(&self, modpack: &Modpack, config_args: &ConfigArgs) -> Result<(), PackrinthError> {
        if modpack_is_dirty(modpack) && !self.allow_dirty {
            return Err(PackrinthError::RepoIsDirtyWhileUpdating);
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
        let mut progress_bar = ProgressBar::new_with_eta(modpack.projects.len() * branches.len());
        if let Some((terminal_size::Width(width), terminal_size::Height(_height))) =
            terminal_size::terminal_size()
        {
            // Decrease width when the terminal is small. Otherwise, take 50 columns.
            // Subtracting results in u16, which is safe to convert to usize.
            #[allow(clippy::as_conversions)]
            progress_bar.set_width(cmp::min(width.saturating_sub(45) as usize, 50));
        }

        for branch_name in branches {
            progress_bar.set_action(branch_name, Color::Blue, Style::Bold);

            let branch_config = BranchConfig::from_directory(&modpack.directory, branch_name)?;
            let mut branch_files =
                match BranchFiles::from_directory(&modpack.directory, branch_name) {
                    Ok(branch_files) => branch_files,
                    Err(_error) => BranchFiles::default(&modpack.directory, branch_name)?,
                };

            // Remove all entries to ensure that there will be no duplicates if the user changes loaders
            branch_files.projects = Vec::new();
            branch_files.files = Vec::new();

            let mut dependencies: Vec<VersionDependency> = Vec::new();

            for project in &modpack.projects {
                if let Some(new_dependencies) = self.update_project(
                    branch_name,
                    &branch_config,
                    &mut branch_files,
                    project.0,
                    project.1,
                    require_all,
                    &mut progress_bar,
                    verbose,
                ) {
                    dependencies.extend(new_dependencies);
                }

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
                        self.update_project(
                            branch_name,
                            &branch_config,
                            &mut branch_files,
                            &project_id,
                            &project_settings,
                            require_all,
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

    // Allow because when calling this function, it is clear what all parameters do.
    #[allow(clippy::too_many_arguments)]
    fn update_project(
        &self,
        branch_name: &String,
        branch_config: &BranchConfig,
        branch_files: &mut BranchFiles,
        project_id: &str,
        project_settings: &ProjectSettings,
        require_all: bool,
        progress_bar: &mut ProgressBar,
        verbose: bool,
    ) -> Option<Vec<VersionDependency>> {
        match File::from_project(
            branch_name,
            branch_config,
            project_id,
            project_settings,
            self.no_beta,
            self.no_alpha,
        ) {
            FileResult::Ok {
                mut file,
                dependencies,
                project_id, // This is the actual id (t234fs23), not the slug (fabric-api)
            } => {
                branch_files.projects.push(BranchFilesProject {
                    name: file.project_name.clone(),
                    id: Some(project_id.to_string()),
                });

                if require_all {
                    file.env = Some(Env {
                        client: SideSupport::Required,
                        server: SideSupport::Required,
                    });
                }

                branch_files.files.push(file);

                if verbose {
                    progress_bar.print_info("added", &project_id, Color::Green, Style::Normal);
                }

                return Some(dependencies);
            }
            FileResult::Skipped(project_id) => {
                if verbose {
                    progress_bar.print_info("skipped", &project_id, Color::Yellow, Style::Normal);
                }
            }
            FileResult::NotFound(project_id) => {
                if verbose {
                    progress_bar.print_info("not found", &project_id, Color::Yellow, Style::Bold);
                }
            }
            FileResult::Err(error) => {
                progress_bar.print_info(
                    "failed",
                    &single_line_error(error.message_and_tip()),
                    Color::Red,
                    Style::Bold,
                );
            }
        }

        None
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
                            "Branch {} is declared in the modpack config file ({}), but it doesn't exist. Please consider removing it from the configuration or re-adding the branch.",
                            branch_name,
                            config::MODPACK_CONFIG_FILE_NAME
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
            match modpack.new_branch(branch_name) {
                Ok(_branch) => (),
                Err(error) => {
                    // Don't use ? because then we can't try again for the next branch.
                    print_error(error.message_and_tip());
                }
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
            modpack.remove_branches(&self.branches);
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

impl DocArgs {
    pub fn run(&self, modpack: &Modpack, config_args: &ConfigArgs) -> Result<(), PackrinthError> {
        match &self.command {
            DocSubCommand::Project(args) => args.run(modpack, config_args),
        }
    }
}

#[derive(Debug)]
struct DocMarkdownTable<'a> {
    column_names: Vec<&'a str>,
    project_map: HashMap<BranchFilesProject, HashMap<String, Option<()>>>,
}

impl ProjectDocArgs {
    // Allow unused self, because then it is clear to the maintainer that self is available for code expansion.
    #[allow(clippy::unused_self)]
    pub fn run(&self, modpack: &Modpack, _config_args: &ConfigArgs) -> Result<(), PackrinthError> {
        let mut column_names = vec!["Name"];
        // project, map: branch, whether it has the project
        let mut project_map: HashMap<BranchFilesProject, HashMap<String, Option<()>>> =
            HashMap::new();

        for branch in &modpack.branches {
            column_names.push(branch);
            // Even tough we are in a loop, we want to abort the action if something goes wrong
            // here, to avoid incorrect docs.
            let branch_files = BranchFiles::from_directory(&modpack.directory, branch)?;

            for project in &branch_files.projects {
                // Vector in hashmap that shows which branches are compatible with a project.
                if let Some(branch_map) = project_map.get_mut(project) {
                    if branch_map.get(branch).is_none() {
                        branch_map.insert(branch.clone(), Some(()));
                    }
                } else {
                    let mut branch_map = HashMap::new();
                    branch_map.insert(branch.clone(), Some(()));
                    project_map.insert(project.clone(), branch_map);
                }
            }
        }

        for project in &mut project_map {
            for branch in &modpack.branches {
                if project.1.get(branch).is_none() {
                    project.1.insert(branch.clone(), None);
                }
            }
        }

        let table = DocMarkdownTable {
            column_names,
            project_map,
        };

        println!("# {} _by {}_", modpack.name, modpack.author);
        println!("{}", modpack.summary);
        println!("## What is included?");
        println!("{table}");

        Ok(())
    }
}

impl Display for DocMarkdownTable<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // Write column names
        writeln!(f, "|{}|", self.column_names.join("|"))?;

        // Write alignment text (:-- is left, :-: is center)
        write!(f, "|:--|")?;
        // Use 1..len because column names include the 'Name' for the project column
        for _ in 1..self.column_names.len() {
            write!(f, ":-:|")?;
        }
        writeln!(f)?;

        let mut sorted_project_map: Vec<_> = self.project_map.iter().collect();
        // Sort by key (human name of project)
        sorted_project_map.sort_by(|a, b| a.0.name.cmp(&b.0.name));

        let mut iter = sorted_project_map.iter().peekable();
        while let Some(project) = iter.next() {
            if let Some(id) = &project.0.id {
                // If project has an id (not a manual file), write a Markdown link.
                let mut project_url = "https://modrinth.com/project/".to_string();
                project_url.push_str(id);
                write!(f, "|[{}]({})|", project.0.name, project_url)?;
            } else {
                write!(f, "|{}|", project.0.name)?;
            }

            let mut sorted_branch_map: Vec<_> = project.1.iter().collect();
            // Sort by key (human name of project)
            sorted_branch_map.sort_by(|a, b| a.0.cmp(b.0));

            for branch in sorted_branch_map {
                let icon = match branch.1 {
                    Some(()) => "✅",
                    None => "❌",
                };
                write!(f, "{icon}|")?;
            }

            // Print newline except for the last time of this loop.
            if iter.peek().is_some() {
                writeln!(f)?;
            }
        }

        Ok(())
    }
}

impl CompletionsArgs {
    // Allow because it is required in Cli::run.
    #[allow(clippy::unnecessary_wraps)]
    pub fn run(&self, _modpack: &Modpack, _config_args: &ConfigArgs) -> Result<(), PackrinthError> {
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
        println!();
        println!(
            "If you find any bugs, have suggestions, or want to contribute, please visit the Git repository at:"
        );
        println!("{}", crate::REPOSITORY);

        Ok(())
    }
}
