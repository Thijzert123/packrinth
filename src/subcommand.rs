use crate::{Cli, print_error, print_success, single_line_error};
use clap::CommandFactory;
use clap_complete::{Generator, shells};
use dialoguer::Confirm;
use indexmap::IndexMap;
use packrinth::config::{
    BranchConfig, BranchFiles, BranchFilesProject, IncludeOrExclude, Modpack, ProjectSettings,
};
use packrinth::modrinth::{Env, File, FileResult, SideSupport};
use packrinth::{PackrinthError, config, modpack_is_dirty};
use progress_bar::pb::ProgressBar;
use progress_bar::{Color, Style};
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use std::path::Path;
use std::{cmp, fs, io};
// Allow because we need all of them
#[allow(clippy::wildcard_imports)]
use crate::cli::*;

impl Cli {
    pub fn run(&mut self) {
        self.subcommand.run(&self.config_args);
    }
}

impl SubCommand {
    fn run(&self, config_args: &ConfigArgs) {
        let current_dir = match &config_args.directory {
            Some(dir) => dir,
            None => match std::env::current_dir() {
                Ok(current_dir) => &current_dir.clone(),
                Err(error) => {
                    print_error(
                        PackrinthError::FailedToGetCurrentDirectory(error.to_string())
                            .message_and_tip(),
                    );
                    return;
                }
            },
        };

        if let Self::Init(args) = self {
            args.run(current_dir, config_args);

            return;
        }

        let mut modpack = match Modpack::from_directory(current_dir) {
            Ok(modpack) => modpack,
            Err(error) => {
                print_error(error.message_and_tip());
                return;
            }
        };

        if modpack.pack_format != config::CURRENT_PACK_FORMAT {
            print_error(PackrinthError::InvalidPackFormat(modpack.pack_format).message_and_tip());
            return;
        }

        match self {
            SubCommand::Init(_args) => (),
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
    pub fn run(&self, directory: &Path, _config_args: &ConfigArgs) {
        let modpack_config_path = directory.join(config::MODPACK_CONFIG_FILE_NAME);
        if !self.force
            && let Ok(exists) = fs::exists(&modpack_config_path)
            && exists
        {
            print_error(
                PackrinthError::ModpackAlreadyExists(directory.display().to_string())
                    .message_and_tip(),
            );
            return;
        }

        let modpack = match Modpack::new(directory) {
            Ok(modpack) => modpack,
            Err(error) => {
                print_error(error.message_and_tip());
                return;
            }
        };

        match modpack.save() {
            Ok(()) => {
                if !self.no_git_repo
                    && let Err(error) = gix::init(directory)
                {
                    // If the repo already exists, don't show an error.
                    if !matches!(
                        &error,
                        gix::init::Error::Init(gix::create::Error::DirectoryExists { path })
                            if path.file_name() == Some(std::ffi::OsStr::new(".git"))
                    ) {
                        print_error(
                            PackrinthError::FailedToInitGitRepoWhileInitModpack(error.to_string())
                                .message_and_tip(),
                        );
                        return;
                    }
                }

                print_success(format!(
                    "created new modpack in directory {}",
                    directory.display()
                ));
            }
            Err(error) => print_error(error.message_and_tip()),
        }
    }
}

impl ProjectArgs {
    pub fn run(&self, modpack: &mut Modpack, config_args: &ConfigArgs) {
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
            ListProjectsArgs::list(&modpack.projects);
        } else {
            ListProjectsArgs::run(&ListProjectsArgs {}, modpack, config_args);
        }
    }
}

impl ListProjectsArgs {
    // Allow unused self, because then it is clear to the maintainer that self is available for code expansion.
    #[allow(clippy::unused_self)]
    pub fn run(&self, modpack: &mut Modpack, _config_args: &ConfigArgs) {
        Self::list(&modpack.projects);
    }

    // Allowing unnecessary return because it makes it clear that using ? in the function is ok.
    #[allow(clippy::unnecessary_wraps)]
    pub fn list(projects: &IndexMap<String, ProjectSettings>) {
        if projects.is_empty() {
            println!("There are no projects added to this modpack yet.");
            return;
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
    }
}

impl AddProjectsArgs {
    pub fn run(&self, modpack: &mut Modpack, _config_args: &ConfigArgs) {
        let include_or_exclude = if let Some(include) = self.inclusions.clone() {
            Some(IncludeOrExclude::Include(include))
        } else {
            self.exclusions.clone().map(IncludeOrExclude::Exclude)
        };

        modpack.add_projects(&self.projects, &None, &include_or_exclude);
        match modpack.save() {
            Ok(()) => print_success(format!("added {}", self.projects.join(", "))),
            Err(error) => print_error(error.message_and_tip()),
        }
    }
}

impl VersionOverrideProjectArgs {
    pub fn run(&self, modpack: &mut Modpack, config_args: &ConfigArgs) {
        match &self.command {
            VersionOverrideSubCommand::Add(args) => args.run(modpack, config_args),
            VersionOverrideSubCommand::Remove(args) => args.run(modpack, config_args),
        }
    }
}

impl AddVersionOverrideArgs {
    pub fn run(&self, modpack: &mut Modpack, _config_args: &ConfigArgs) {
        if let Err(error) =
            modpack.add_project_override(&self.project, &self.branch, &self.project_version_id)
        {
            print_error(error.message_and_tip());
            return;
        }
        match modpack.save() {
            Ok(()) => print_success(format!(
                "added override for {}, branch {} and version ID {}",
                self.project, self.branch, self.project_version_id
            )),
            Err(error) => print_error(error.message_and_tip()),
        }
    }
}

impl RemoveVersionOverrideArgs {
    pub fn run(&self, modpack: &mut Modpack, _config_args: &ConfigArgs) {
        if self.all {
            if let Err(error) = modpack.remove_all_project_overrides(&self.project) {
                print_error(error.message_and_tip());
                return;
            }

            match modpack.save() {
                Ok(()) => print_success(format!("removed all overrides for {}", self.project)),
                Err(error) => print_error(error.message_and_tip()),
            }
        } else if let Some(branch) = &self.branch {
            if let Err(error) = modpack.remove_project_override(branch, &self.project) {
                print_error(error.message_and_tip());
                return;
            }

            match modpack.save() {
                Ok(()) => {
                    print_success(format!("removed {} override for {}", self.project, branch));
                }
                Err(error) => print_error(error.message_and_tip()),
            }
        } else {
            print_error(PackrinthError::NoBranchSpecified.message_and_tip());
        }
    }
}

impl InclusionsProjectArgs {
    pub fn run(&self, modpack: &mut Modpack, config_args: &ConfigArgs) {
        match &self.command {
            InclusionsSubCommand::Add(args) => args.run(modpack, config_args),
            InclusionsSubCommand::Remove(args) => args.run(modpack, config_args),
        }
    }
}

impl AddInclusionsArgs {
    pub fn run(&self, modpack: &mut Modpack, _config_args: &ConfigArgs) {
        if let Err(error) = modpack.add_project_inclusions(&self.project, &self.inclusions) {
            print_error(error.message_and_tip());
            return;
        }

        match modpack.save() {
            Ok(()) => print_success(format!(
                "added {} inclusions for {}",
                self.inclusions.join(", "),
                self.project
            )),
            Err(error) => print_error(error.message_and_tip()),
        }
    }
}

impl RemoveInclusionsArgs {
    pub fn run(&self, modpack: &mut Modpack, _config_args: &ConfigArgs) {
        if self.all {
            if let Err(error) = modpack.remove_all_project_inclusions(&self.project) {
                print_error(error.message_and_tip());
                return;
            }

            match modpack.save() {
                Ok(()) => print_success(format!("removed all inclusions for {}", self.project)),
                Err(error) => print_error(error.message_and_tip()),
            }
        } else if let Some(inclusions) = &self.inclusions {
            if let Err(error) = modpack.remove_project_inclusions(&self.project, inclusions) {
                print_error(error.message_and_tip());
                return;
            }

            match modpack.save() {
                Ok(()) => print_success(format!(
                    "removed {} inclusions for {}",
                    inclusions.join(", "),
                    self.project
                )),
                Err(error) => print_error(error.message_and_tip()),
            }
        } else {
            print_error(PackrinthError::NoInclusionsSpecified.message_and_tip());
        }
    }
}

impl ExclusionsProjectArgs {
    pub fn run(&self, modpack: &mut Modpack, config_args: &ConfigArgs) {
        match &self.command {
            ExclusionsSubCommand::Add(args) => args.run(modpack, config_args),
            ExclusionsSubCommand::Remove(args) => args.run(modpack, config_args),
        }
    }
}

impl AddExclusionsArgs {
    pub fn run(&self, modpack: &mut Modpack, _config_args: &ConfigArgs) {
        if let Err(error) = modpack.add_project_exclusions(&self.project, &self.exclusions) {
            print_error(error.message_and_tip());
            return;
        }

        match modpack.save() {
            Ok(()) => print_success(format!(
                "added {} exclusions for {}",
                self.exclusions.join(", "),
                self.project
            )),
            Err(error) => print_error(error.message_and_tip()),
        }
    }
}

impl RemoveExclusionsArgs {
    pub fn run(&self, modpack: &mut Modpack, _config_args: &ConfigArgs) {
        if self.all {
            if let Err(error) = modpack.remove_all_project_exclusions(&self.project) {
                print_error(error.message_and_tip());
                return;
            }

            match modpack.save() {
                Ok(()) => print_success(format!("removed all exclusions for {}", self.project)),
                Err(error) => print_error(error.message_and_tip()),
            }
        } else if let Some(exclusions) = &self.exclusions {
            if let Err(error) = modpack.remove_project_exclusions(&self.project, exclusions) {
                print_error(error.message_and_tip());
                return;
            }

            match modpack.save() {
                Ok(()) => print_success(format!(
                    "removed {} exclusions for {}",
                    exclusions.join(", "),
                    self.project
                )),
                Err(error) => print_error(error.message_and_tip()),
            }
        } else {
            print_error(PackrinthError::NoExclusionsSpecified.message_and_tip());
        }
    }
}

impl RemoveProjectsArgs {
    pub fn run(&self, modpack: &mut Modpack, _config_args: &ConfigArgs) {
        modpack.remove_projects(&self.projects);

        match modpack.save() {
            Ok(()) => print_success(format!("removed {}", self.projects.join(", "))),
            Err(error) => print_error(error.message_and_tip()),
        }
    }
}
// TODO default flags in modpack.json
impl UpdateArgs {
    pub fn run(&self, modpack: &Modpack, config_args: &ConfigArgs) {
        if modpack_is_dirty(modpack) && !self.allow_dirty {
            print_error(PackrinthError::RepoIsDirtyWhileUpdating.message_and_tip());
            return;
        }

        let branches = if let Some(branches) = &self.branches {
            branches
        } else {
            &modpack.branches
        };

        if let Err(error) = Self::update_branches(
            modpack,
            branches,
            self.no_beta,
            self.no_alpha,
            self.require_all,
            config_args.verbose,
        ) {
            print_error(error.message_and_tip());
        }
    }

    // Allow because when this function is called, it is apparent what the bools mean.
    #[allow(clippy::fn_params_excessive_bools)]
    // Allow because most of it is by Cargo fmt.
    #[allow(clippy::too_many_lines)]
    fn update_branches(
        modpack: &Modpack,
        branches: &Vec<String>,
        no_beta: bool,
        no_alpha: bool,
        require_all: bool,
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

            for project in &modpack.projects {
                let project_id = project.0;
                let project_settings = project.1;
                match File::from_project(
                    branch_name,
                    &branch_config,
                    project_id,
                    project_settings,
                    no_beta,
                    no_alpha,
                ) {
                    FileResult::Ok(mut file) => {
                        branch_files.projects.push(BranchFilesProject {
                            name: file.project_name.clone(),
                            id: Some(project_id.clone()),
                        });

                        if require_all {
                            file.env = Some(Env {
                                client: SideSupport::Required,
                                server: SideSupport::Required,
                            });
                        }

                        branch_files.files.push(file);

                        if verbose {
                            progress_bar.print_info(
                                "added",
                                project_id,
                                Color::Green,
                                Style::Normal,
                            );
                        }
                    }
                    FileResult::Skipped(project_id) => {
                        if verbose {
                            progress_bar.print_info(
                                "skipped",
                                &project_id,
                                Color::Yellow,
                                Style::Normal,
                            );
                        }
                    }
                    FileResult::NotFound(project_id) => {
                        if verbose {
                            progress_bar.print_info(
                                "not found",
                                &project_id,
                                Color::Yellow,
                                Style::Bold,
                            );
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

                progress_bar.inc();
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
}

impl BranchArgs {
    pub fn run(&self, modpack: &mut Modpack, config_args: &ConfigArgs) {
        if let Some(command) = &self.command {
            match command {
                BranchSubCommand::List(args) => args.run(modpack, config_args),
                BranchSubCommand::Add(args) => args.run(modpack, config_args),
                BranchSubCommand::Remove(args) => args.run(modpack, config_args),
            }
        } else if let Some(branch_names) = &self.branches {
            ListBranchesArgs::list(&modpack.directory, branch_names);
        } else {
            ListBranchesArgs::run(&ListBranchesArgs {}, modpack, config_args);
        }
    }
}

impl ListBranchesArgs {
    // Allow unused self, because then it is clear to the maintainer that self is available for code expansion.
    #[allow(clippy::unused_self)]
    pub fn run(&self, modpack: &Modpack, _config_args: &ConfigArgs) {
        Self::list(&modpack.directory, &modpack.branches);
    }

    pub fn list(directory: &Path, branches: &[String]) {
        if branches.is_empty() {
            println!("There are no branches added to this modpack yet.");
            return;
        }

        let mut iter = branches.iter().peekable();
        while let Some(branch_name) = iter.next() {
            match BranchConfig::from_directory(directory, branch_name) {
                Ok(branch) => branch.print_display(branch_name),
                Err(error) => {
                    if let PackrinthError::BranchDoesNotExist(_branch_name) = error {
                        println!(
                            "Branch {} is declared in the modpack config file ({}), but it doesn't exist. Please consider removing it from the configuration or re-adding the branch.",
                            branch_name,
                            config::MODPACK_CONFIG_FILE_NAME
                        );
                    } else {
                        print_error(error.message_and_tip());
                    }
                }
            }

            // Print new line between branches, but not at the very end.
            if iter.peek().is_some() {
                println!();
            }
        }
    }
}

impl AddBranchesArgs {
    pub fn run(&self, modpack: &mut Modpack, _config_args: &ConfigArgs) {
        for branch_name in &self.branches {
            match modpack.new_branch(branch_name) {
                Ok(_branch) => (),
                Err(error) => {
                    print_error(error.message_and_tip());
                    return;
                }
            }
        }

        match modpack.save() {
            Ok(()) => print_success(format!("added branches {}", self.branches.join(", "))),
            Err(error) => print_error(error.message_and_tip()),
        }
    }
}

impl RemoveBranchesArgs {
    pub fn run(&self, modpack: &mut Modpack, _config_args: &ConfigArgs) {
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
    }
}

impl ExportArgs {
    pub fn run(&self, modpack: &Modpack, _config_args: &ConfigArgs) {
        match &self.branches {
            None => Self::export_branches(modpack, &modpack.branches),
            Some(branches) => Self::export_branches(modpack, branches),
        }
    }

    fn export_branches(modpack: &Modpack, branches: &Vec<String>) {
        for branch in branches {
            match modpack.export_branch(branch) {
                Ok(modpack_path) => {
                    print_success(format!("exported {} to {}", branch, modpack_path.display()));
                }
                Err(error) => print_error(error.message_and_tip()),
            }
        }
    }
}

impl DocArgs {
    pub fn run(&self, modpack: &Modpack, config_args: &ConfigArgs) {
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
    pub fn run(&self, modpack: &Modpack, _config_args: &ConfigArgs) {
        let mut column_names = vec!["Name"];
        // project, map: branch, whether it has the project
        let mut project_map: HashMap<BranchFilesProject, HashMap<String, Option<()>>> =
            HashMap::new();

        for branch in &modpack.branches {
            column_names.push(branch);
            let branch_files = match BranchFiles::from_directory(&modpack.directory, branch) {
                Ok(branch_files) => branch_files,
                Err(error) => {
                    print_error(error.message_and_tip());
                    return;
                }
            };

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
    pub fn run(&self, _modpack: &Modpack, _config_args: &ConfigArgs) {
        let mut cmd = Cli::command();
        match self.shell {
            CompletionShell::Bash => Self::print_completions(shells::Bash, &mut cmd),
            CompletionShell::Elvish => Self::print_completions(shells::Elvish, &mut cmd),
            CompletionShell::Fish => Self::print_completions(shells::Fish, &mut cmd),
            CompletionShell::PowerShell => Self::print_completions(shells::PowerShell, &mut cmd),
            CompletionShell::Zsh => Self::print_completions(shells::Zsh, &mut cmd),
        }
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
