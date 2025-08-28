use crate::{ConfigArgs, print_error, print_success, single_line_error};
use clap::Parser;
use dialoguer::Confirm;
use packrinth::config::{
    BranchConfig, BranchFiles, BranchFilesProject, IncludeOrExclude, Modpack, ProjectSettings,
};
use packrinth::modrinth::{File, FileResult};
use packrinth::{PackrinthError, config};
use progress_bar::pb::ProgressBar;
use progress_bar::{Color, Style};
use std::cmp;
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use std::path::Path;

#[derive(Debug, Parser)]
pub struct ProjectArgs {
    #[clap(subcommand)]
    command: Option<ProjectSubCommand>,

    /// List information about added projects. If none are specified, all projects will be listed.
    projects: Option<Vec<String>>,
}

#[derive(Parser, Debug)]
enum ProjectSubCommand {
    /// List all projects that are currently added to this modpack
    #[clap(alias = "ls")]
    List(ListProjectsArgs),

    /// Add projects to this modpack
    Add(AddProjectsArgs),

    /// Add a version override to a project in this modpack
    Override(OverrideProjectArgs),

    /// Add an include list to a project in this modpack
    Include(IncludeProjectArgs),

    /// Add an exclude list to a project in this modpack
    Exclude(ExcludeProjectArgs),

    /// Remove projects from this modpack
    #[clap(alias = "rm")]
    Remove(RemoveProjectsArgs),
}

#[derive(Parser, Debug)]
struct ListProjectsArgs;

#[derive(Parser, Debug)]
struct AddProjectsArgs {
    /// Projects to add
    ///
    /// The projects must be from Modrinth. You have to specify either the human-readable
    /// slug that appears in the URL (fabric-api) or the slug (`P7dR8mSH`).
    projects: Vec<String>,

    #[clap(short, long, group = "include_or_exclude")]
    inclusions: Option<Vec<String>>,

    #[clap(short, long, group = "include_or_exclude")]
    exclusions: Option<Vec<String>>,
}

#[derive(Parser, Debug)]
struct OverrideProjectArgs {
    #[clap(subcommand)]
    command: OverrideSubCommand,
}

#[derive(Parser, Debug)]
enum OverrideSubCommand {
    Add(AddOverrideArgs),
    Remove(RemoveOverrideArgs),
}

#[derive(Parser, Debug)]
struct AddOverrideArgs {
    project: String,

    branch: String,

    project_version_id: String,
}

#[derive(Parser, Debug)]
struct RemoveOverrideArgs {
    project: String,

    branch: Option<String>,

    #[clap(short, long)]
    all: bool,
}

#[derive(Parser, Debug)]
struct IncludeProjectArgs {
    #[clap(subcommand)]
    command: IncludeSubCommand,
}

#[derive(Parser, Debug)]
enum IncludeSubCommand {
    Add(AddInclusionsArgs),

    #[clap(alias = "rm")]
    Remove(RemoveInclusionsArgs),
}

#[derive(Parser, Debug)]
struct AddInclusionsArgs {
    project: String,

    inclusions: Vec<String>,
}

#[derive(Parser, Debug)]
struct RemoveInclusionsArgs {
    project: String,

    inclusions: Option<Vec<String>>,

    #[clap(short, long)]
    all: bool,
}

#[derive(Parser, Debug)]
struct ExcludeProjectArgs {
    #[clap(subcommand)]
    command: ExcludeSubCommand,
}

#[derive(Parser, Debug)]
enum ExcludeSubCommand {
    Add(AddExclusionsArgs),

    #[clap(alias = "rm")]
    Remove(RemoveExclusionsArgs),
}

#[derive(Parser, Debug)]
struct AddExclusionsArgs {
    project: String,

    exclusions: Vec<String>,
}

#[derive(Parser, Debug)]
struct RemoveExclusionsArgs {
    project: String,

    exclusions: Option<Vec<String>>,

    #[clap(short, long)]
    all: bool,
}

#[derive(Parser, Debug)]
struct RemoveProjectsArgs {
    projects: Vec<String>,
}

#[derive(Debug, Parser)]
pub struct UpdateArgs {
    branches: Option<Vec<String>>,

    #[clap(long)]
    no_alpha: bool,

    #[clap(long)]
    no_beta: bool,
}

#[derive(Debug, Parser)]
pub struct BranchArgs {
    #[clap(subcommand)]
    command: Option<BranchSubCommand>,

    branches: Option<Vec<String>>,
}

#[derive(Parser, Debug)]
enum BranchSubCommand {
    #[clap(alias = "ls")]
    List(ListBranchesArgs),

    Add(AddBranchesArgs),

    #[clap(alias = "rm")]
    Remove(RemoveBranchesArgs),
}

#[derive(Parser, Debug)]
struct ListBranchesArgs;

#[derive(Parser, Debug)]
struct AddBranchesArgs {
    branches: Vec<String>,
}

#[derive(Parser, Debug)]
struct RemoveBranchesArgs {
    branches: Vec<String>,
}

#[derive(Parser, Debug)]
pub struct ExportArgs {
    branch: String,
}

#[derive(Parser, Debug)]
pub struct DocArgs {
    #[clap(subcommand)]
    command: DocSubCommand,
}

#[derive(Parser, Debug)]
enum DocSubCommand {
    Project(ProjectDocArgs),
}

#[derive(Parser, Debug)]
struct ProjectDocArgs;

#[derive(Debug)]
struct DocMarkdownTable<'a> {
    column_names: Vec<&'a str>,
    project_map: HashMap<BranchFilesProject, HashMap<String, Option<()>>>,
}

impl ProjectArgs {
    pub fn run(&self, modpack: &mut Modpack, config_args: &ConfigArgs) {
        if let Some(command) = &self.command {
            match command {
                ProjectSubCommand::List(args) => args.run(modpack, config_args),
                ProjectSubCommand::Add(args) => args.run(modpack, config_args),
                ProjectSubCommand::Override(args) => args.run(modpack, config_args),
                ProjectSubCommand::Include(args) => args.run(modpack, config_args),
                ProjectSubCommand::Exclude(args) => args.run(modpack, config_args),
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
    pub fn list(projects: &HashMap<String, ProjectSettings>) {
        if projects.is_empty() {
            println!("There are no projects added to this modpack yet.");
            return;
        }

        let mut iter = projects.iter().peekable();
        while let Some(project) = iter.next() {
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

            // Print new line between projects, but not at the very end.
            if iter.peek().is_some() {
                println!();
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

impl OverrideProjectArgs {
    pub fn run(&self, modpack: &mut Modpack, config_args: &ConfigArgs) {
        match &self.command {
            OverrideSubCommand::Add(args) => args.run(modpack, config_args),
            OverrideSubCommand::Remove(args) => args.run(modpack, config_args),
        }
    }
}

impl AddOverrideArgs {
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

impl RemoveOverrideArgs {
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
            print_error((
                "no branch specified",
                "specify a branch or remove all with the --all flag",
            ));
        }
    }
}

impl IncludeProjectArgs {
    pub fn run(&self, modpack: &mut Modpack, config_args: &ConfigArgs) {
        match &self.command {
            IncludeSubCommand::Add(args) => args.run(modpack, config_args),
            IncludeSubCommand::Remove(args) => args.run(modpack, config_args),
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
            print_error((
                "no inclusions specified",
                "specify inclusions or remove all with the --all flag",
            ));
        }
    }
}

impl ExcludeProjectArgs {
    pub fn run(&self, modpack: &mut Modpack, config_args: &ConfigArgs) {
        match &self.command {
            ExcludeSubCommand::Add(args) => args.run(modpack, config_args),
            ExcludeSubCommand::Remove(args) => args.run(modpack, config_args),
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

impl UpdateArgs {
    pub fn run(&self, modpack: &Modpack, config_args: &ConfigArgs) {
        if let Some(branches) = &self.branches {
            for branch in branches {
                if let Err(error) = Self::update_branch(
                    modpack,
                    branch,
                    self.no_beta,
                    self.no_alpha,
                    config_args.verbose,
                ) {
                    print_error(error.message_and_tip());
                    return;
                }
            }

            print_success(format!("updated {}", branches.join(", ")));
        } else {
            for branch in &modpack.branches {
                if let Err(error) = Self::update_branch(
                    modpack,
                    branch,
                    self.no_beta,
                    self.no_alpha,
                    config_args.verbose,
                ) {
                    print_error(error.message_and_tip());
                    return;
                }
            }

            println!();
            print_success(format!("updated {}", modpack.branches.join(", ")));
        }
    }

    fn update_branch(
        modpack: &Modpack,
        branch_name: &String,
        no_beta: bool,
        no_alpha: bool,
        verbose: bool,
    ) -> Result<(), PackrinthError> {
        let branch_config = BranchConfig::from_directory(&modpack.directory, branch_name)?;
        let mut branch_files = BranchFiles::from_directory_allow_new(&modpack.directory, branch_name)?;

        // Remove all entries to ensure that there will be no duplicates if the user changes loaders
        branch_files.files = Vec::new();

        let mut progress_bar = ProgressBar::new_with_eta(modpack.projects.len());
        progress_bar.set_action(branch_name, Color::Blue, Style::Bold);
        if let Some((terminal_size::Width(width), terminal_size::Height(_height))) =
            terminal_size::terminal_size()
        {
            // Decrease width when the terminal is small. Otherwise, take 50 columns.
            // Subtracting results in u16, which is safe to convert to usize.
            #[allow(clippy::as_conversions)]
            progress_bar.set_width(cmp::min(width.saturating_sub(45) as usize, 50));
        }

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
                FileResult::Ok(file) => {
                    branch_files.projects.push(BranchFilesProject {
                        name: file.project_name.clone(),
                        id: project_id.clone(),
                    });
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

        progress_bar.finalize();

        branch_files.save(&modpack.directory, branch_name)
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
        match modpack.export(&self.branch) {
            Ok(modpack_path) => print_success(format!(
                "exported {} to {}",
                self.branch,
                modpack_path.display()
            )),
            Err(error) => print_error(error.message_and_tip()),
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
                    project_map.insert(
                        project.clone(),
                        branch_map,
                    );
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
            let mut project_url = "https://modrinth.com/project/".to_string();
            project_url.push_str(&project.0.id);
            write!(f, "|[{}]({})|", project.0.name, project_url)?;

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
