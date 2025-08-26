use crate::ConfigArgs;
use anyhow::{Context, Result, bail};
use clap::Parser;
use dialoguer::Confirm;
use packrinth::config;
use packrinth::config::{BranchConfig, BranchFiles, IncludeOrExclude, Modpack, ProjectSettings};
use packrinth::modrinth::{File, FileError};
use progress_bar::pb::ProgressBar;
use progress_bar::{Color, Style};
use std::cmp;
use std::collections::HashMap;
use std::fmt::Debug;
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

    minecraft_version: String,

    project_version_id: String,
}

#[derive(Parser, Debug)]
struct RemoveOverrideArgs {
    project: String,

    minecraft_version: Option<String>,

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

    inclusions: Vec<String>,

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

    exclusions: Vec<String>,

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

impl ProjectArgs {
    pub fn run(&self, modpack: &mut Modpack, config_args: &ConfigArgs) -> Result<()> {
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
            ListProjectsArgs::list(&modpack.projects)
        } else {
            ListProjectsArgs::run(&ListProjectsArgs {}, modpack, config_args)
        }
    }
}

impl ListProjectsArgs {
    // Allow unused self, because then it is clear to the maintainer that self is available for code expansion.
    #[allow(clippy::unused_self)]
    pub fn run(&self, modpack: &mut Modpack, _config_args: &ConfigArgs) -> Result<()> {
        Self::list(&modpack.projects)
    }

    // Allowing unnecessary return because it makes it clear that using ? in the function is ok.
    #[allow(clippy::unnecessary_wraps)]
    pub fn list(projects: &HashMap<String, ProjectSettings>) -> Result<()> {
        if projects.is_empty() {
            println!("There are no projects added to this modpack yet.");
            return Ok(());
        }

        let mut iter = projects.iter().peekable();
        while let Some(project) = iter.next() {
            println!("{}", project.0);

            // if let Some(project_settings) = project.1 {
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
            // }

            // Print new line between projects, but not at the very end.
            if iter.peek().is_some() {
                println!();
            }
        }

        Ok(())
    }
}

impl AddProjectsArgs {
    pub fn run(&self, modpack: &mut Modpack, _config_args: &ConfigArgs) -> Result<()> {
        let include_or_exclude = if let Some(include) = self.inclusions.clone() {
            Some(IncludeOrExclude::Include(include))
        } else {
            self.exclusions.clone().map(IncludeOrExclude::Exclude)
        };

        modpack.add_projects(&self.projects, &None, &include_or_exclude)?;

        Ok(())
    }
}

impl OverrideProjectArgs {
    pub fn run(&self, modpack: &mut Modpack, config_args: &ConfigArgs) -> Result<()> {
        match &self.command {
            OverrideSubCommand::Add(args) => args.run(modpack, config_args),
            OverrideSubCommand::Remove(args) => args.run(modpack, config_args),
        }
    }
}

impl AddOverrideArgs {
    pub fn run(&self, modpack: &mut Modpack, _config_args: &ConfigArgs) -> Result<()> {
        modpack.add_project_override(
            &self.project,
            &self.minecraft_version,
            &self.project_version_id,
        )
    }
}

impl RemoveOverrideArgs {
    pub fn run(&self, modpack: &mut Modpack, _config_args: &ConfigArgs) -> Result<()> {
        if self.all {
            modpack.remove_all_project_overrides(&self.project)
        } else if let Some(minecraft_version) = &self.minecraft_version {
            modpack.remove_project_override(&self.project, minecraft_version)
        } else {
            bail!("Please add a Minecraft version or remove all overrides by adding --all flag")
        }
    }
}

impl IncludeProjectArgs {
    pub fn run(&self, modpack: &mut Modpack, config_args: &ConfigArgs) -> Result<()> {
        match &self.command {
            IncludeSubCommand::Add(args) => args.run(modpack, config_args),
            IncludeSubCommand::Remove(args) => args.run(modpack, config_args),
        }
    }
}

impl AddInclusionsArgs {
    pub fn run(&self, modpack: &mut Modpack, _config_args: &ConfigArgs) -> Result<()> {
        modpack.add_project_inclusions(&self.project, &self.inclusions)
    }
}

impl RemoveInclusionsArgs {
    pub fn run(&self, modpack: &mut Modpack, _config_args: &ConfigArgs) -> Result<()> {
        if self.all {
            modpack.remove_all_project_inclusions(&self.project)
        } else {
            modpack.remove_project_inclusions(&self.project, &self.inclusions)
        }
    }
}

impl ExcludeProjectArgs {
    pub fn run(&self, modpack: &mut Modpack, config_args: &ConfigArgs) -> Result<()> {
        match &self.command {
            ExcludeSubCommand::Add(args) => args.run(modpack, config_args),
            ExcludeSubCommand::Remove(args) => args.run(modpack, config_args),
        }
    }
}

impl AddExclusionsArgs {
    pub fn run(&self, modpack: &mut Modpack, _config_args: &ConfigArgs) -> Result<()> {
        modpack.add_project_exclusions(&self.project, &self.exclusions)
    }
}

impl RemoveExclusionsArgs {
    pub fn run(&self, modpack: &mut Modpack, _config_args: &ConfigArgs) -> Result<()> {
        if self.all {
            modpack.remove_all_project_exclusions(&self.project)
        } else {
            modpack.remove_project_exclusions(&self.project, &self.exclusions)
        }
    }
}

impl RemoveProjectsArgs {
    pub fn run(&self, modpack: &mut Modpack, _config_args: &ConfigArgs) -> Result<()> {
        modpack.remove_projects(&self.projects)
    }
}

impl UpdateArgs {
    pub fn run(&self, modpack: &Modpack, config_args: &ConfigArgs) -> Result<()> {
        if let Some(branches) = &self.branches {
            for branch in branches {
                Self::update_branch(
                    modpack,
                    branch,
                    self.no_beta,
                    self.no_alpha,
                    config_args.verbose,
                )?;
            }
        } else {
            for branch in &modpack.branches {
                Self::update_branch(
                    modpack,
                    branch,
                    self.no_beta,
                    self.no_alpha,
                    config_args.verbose,
                )?;
            }
        }

        Ok(())
    }

    fn update_branch(
        modpack: &Modpack,
        branch_name: &String,
        no_beta: bool,
        no_alpha: bool,
        verbose: bool,
    ) -> Result<()> {
        let branch_config = BranchConfig::from_directory(&modpack.directory, branch_name)?;
        let mut branch_files = BranchFiles::from_directory(&modpack.directory, branch_name)?;

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
                Ok(file) => {
                    branch_files.files.push(file);

                    if verbose {
                        progress_bar.print_info(
                            "Added",
                            project_id,
                            Color::LightGreen,
                            Style::Normal,
                        );
                    }
                }
                Err(error) if error.downcast_ref::<FileError>().is_some() => {
                    match error.downcast_ref::<FileError>().unwrap() {
                        FileError::Skipped(_project_id) => {
                            if verbose {
                                progress_bar.print_info(
                                    "Skipped",
                                    project_id,
                                    Color::Yellow,
                                    Style::Normal,
                                );
                            }
                        }
                        FileError::NotFound(_project_id) => progress_bar.print_info(
                            "Not found",
                            project_id,
                            Color::Yellow,
                            Style::Bold,
                        ),
                    }
                }
                Err(error) => {
                    progress_bar.print_info("Failed", &error.to_string(), Color::Red, Style::Bold);
                }
            }

            progress_bar.inc();
        }

        progress_bar.print_final_info(branch_name, "updated", Color::LightGreen, Style::Bold);

        branch_files.save(&modpack.directory, branch_name)
    }
}

impl BranchArgs {
    pub fn run(&self, modpack: &mut Modpack, config_args: &ConfigArgs) -> Result<()> {
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
    pub fn run(&self, modpack: &Modpack, _config_args: &ConfigArgs) -> Result<()> {
        Self::list(&modpack.directory, &modpack.branches)
    }

    pub fn list(directory: &Path, branches: &[String]) -> Result<()> {
        if branches.is_empty() {
            println!("There are no branches added to this modpack yet.");
            return Ok(());
        }

        let mut iter = branches.iter().peekable();
        while let Some(branch_name) = iter.next() {
            match BranchConfig::from_directory(directory, branch_name).with_context(|| {
                format!(
                    "Failed to get branch {} in directory {}",
                    branch_name,
                    directory.display()
                )
            }) {
                Ok(branch) => branch.print_display(branch_name),
                Err(error) => {
                    if let Some(error) = error.downcast_ref::<std::io::Error>()
                        && error.kind() == std::io::ErrorKind::NotFound
                    {
                        eprintln!(
                            "Branch {} is declared in the modpack config file ({}), but it doesn't exist. Please consider removing it from the configuration or re-adding the branch.",
                            branch_name,
                            config::MODPACK_CONFIG_FILE_NAME
                        );
                    } else {
                        bail!(error);
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
    pub fn run(&self, modpack: &mut Modpack, _config_args: &ConfigArgs) -> Result<()> {
        let mut iter = self.branches.iter().peekable();
        while let Some(branch_name) = iter.next() {
            let new_branch = modpack.new_branch(branch_name)?;
            new_branch.print_display(branch_name);

            // Print new line between branches, but not at the very end.
            if iter.peek().is_some() {
                println!();
            }
        }

        Ok(())
    }
}

impl RemoveBranchesArgs {
    pub fn run(&self, modpack: &mut Modpack, _config_args: &ConfigArgs) -> Result<()> {
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
            if self.branches.len() == 1 {
                println!("Removed {} branch", self.branches.len());
            } else {
                println!("Removed {} branches", self.branches.len());
            }

            Ok(())
        } else {
            println!("Aborted action");
            Ok(())
        }
    }
}

impl ExportArgs {
    pub fn run(&self, modpack: &mut Modpack, _config_args: &ConfigArgs) -> Result<()> {
        if let Ok(mrpack_path) = modpack.export(&self.branch) {
            println!(
                "Exported branch {} to {}",
                &self.branch,
                mrpack_path.display()
            );
            Ok(())
        } else {
            bail!("Failed to export branch {}", &self.branch);
        }
    }
}
