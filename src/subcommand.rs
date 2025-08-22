use crate::ConfigArgs;
use crate::json::config;
use crate::json::config::{Branch, IncludeOrExclude, Modpack, ProjectSettings};
use anyhow::{Context, Result, bail};
use clap::Parser;
use dialoguer::Confirm;
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Parser)]
pub struct ProjectArgs {
    #[clap(subcommand)]
    command: Option<ProjectSubCommand>,

    projects: Option<Vec<String>>,
}

#[derive(Parser, Debug)]
enum ProjectSubCommand {
    #[clap(alias = "ls")]
    List(ListProjectsArgs),

    Add(AddProjectsArgs),

    Override(OverrideProjectArgs),

    Include(IncludeProjectArgs),

    Exclude(ExcludeProjectArgs),

    #[clap(alias = "rm")]
    Remove(RemoveProjectsArgs),
}

#[derive(Parser, Debug)]
struct ListProjectsArgs;

#[derive(Parser, Debug)]
struct AddProjectsArgs {
    projects: Vec<String>,

    #[clap(short, long, group = "include_or_exclude")]
    include: Option<Vec<String>>,

    #[clap(short, long, group = "include_or_exclude")]
    exclude: Option<Vec<String>>,
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
    Add(AddIncludesArgs),

    #[clap(alias = "rm")]
    Remove(RemoveIncludesArgs),
}

#[derive(Parser, Debug)]
struct AddIncludesArgs {
    project: String,

    includes: Vec<String>,
}

#[derive(Parser, Debug)]
struct RemoveIncludesArgs {
    project: String,

    includes: Vec<String>,

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
    Add(AddExcludesArgs),

    #[clap(alias = "rm")]
    Remove(RemoveExcludesArgs),
}

#[derive(Parser, Debug)]
struct AddExcludesArgs {
    project: String,

    excludes: Vec<String>,
}

#[derive(Parser, Debug)]
struct RemoveExcludesArgs {
    project: String,

    excludes: Vec<String>,

    #[clap(short, long)]
    all: bool,
}

#[derive(Parser, Debug)]
struct RemoveProjectsArgs {
    projects: Vec<String>,
}

#[derive(Debug, Parser)]
pub struct UpdateArgs {
    branch: Option<String>,
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
            let project_map = project_names
                .iter()
                .map(|x| {
                    (
                        x.clone(),
                        ProjectSettings {
                            version_overrides: None,
                            include_or_exclude: None,
                        },
                    )
                })
                .collect();
            ListProjectsArgs::list(&project_map)
        } else {
            ListProjectsArgs::run(&ListProjectsArgs {}, modpack, config_args)
        }
    }
}

impl ListProjectsArgs {
    pub fn run(&self, modpack: &mut Modpack, _: &ConfigArgs) -> Result<()> {
        Self::list(&modpack.projects)
    }

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
                    IncludeOrExclude::Include(includes) => {
                        println!("  - Includes: {}", includes.join(", "))
                    }
                    IncludeOrExclude::Exclude(excludes) => {
                        println!("  - Excludes: {}", excludes.join(", "))
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
    pub fn run(&self, modpack: &mut Modpack, _: &ConfigArgs) -> Result<()> {
        let include_or_exclude = if let Some(include) = self.include.clone() {
            Some(IncludeOrExclude::Include(include))
        } else {
            self.exclude.clone().map(IncludeOrExclude::Exclude)
        };

        modpack.add_projects(&self.projects, None, include_or_exclude)?;

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
    pub fn run(&self, modpack: &mut Modpack, _: &ConfigArgs) -> Result<()> {
        modpack.add_project_override(
            &self.project,
            &self.minecraft_version,
            &self.project_version_id,
        )
    }
}

impl RemoveOverrideArgs {
    pub fn run(&self, modpack: &mut Modpack, _: &ConfigArgs) -> Result<()> {
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

impl AddIncludesArgs {
    pub fn run(&self, modpack: &mut Modpack, _: &ConfigArgs) -> Result<()> {
        modpack.add_project_includes(&self.project, &self.includes)
    }
}

impl RemoveIncludesArgs {
    pub fn run(&self, modpack: &mut Modpack, _: &ConfigArgs) -> Result<()> {
        if self.all {
            modpack.remove_all_project_includes(&self.project)
        } else {
            modpack.remove_project_includes(&self.project, &self.includes)
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

impl AddExcludesArgs {
    pub fn run(&self, modpack: &mut Modpack, _: &ConfigArgs) -> Result<()> {
        modpack.add_project_excludes(&self.project, &self.excludes)
    }
}

impl RemoveExcludesArgs {
    pub fn run(&self, modpack: &mut Modpack, _: &ConfigArgs) -> Result<()> {
        if self.all {
            modpack.remove_all_project_excludes(&self.project)
        } else {
            modpack.remove_project_excludes(&self.project, &self.excludes)
        }
    }
}

impl RemoveProjectsArgs {
    pub fn run(&self, modpack: &mut Modpack, _: &ConfigArgs) -> Result<()> {
        modpack.remove_projects(&self.projects)
    }
}

impl UpdateArgs {
    pub fn run(&self, modpack: &Modpack, config_args: &ConfigArgs) -> Result<()> {
        if self.branch.is_none() {
            for project in &modpack.projects {
                // let thing = Branch::from_working_dir(modpack, &"test".to_string(), false);
                // let thing = newest_version_for_project(project.0, vec!["fabric".to_string()], vec!["1.21.1".to_string()]);
                // println!("{thing:?}")
            }
        }

        Ok(())
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
    pub fn run(&self, modpack: &Modpack, _: &ConfigArgs) -> Result<()> {
        Self::list(&modpack.directory, &modpack.branches)
    }

    pub fn list(directory: &Path, branches: &[String]) -> Result<()> {
        if branches.is_empty() {
            println!("There are no branches added to this modpack yet.");
            return Ok(());
        }

        let mut iter = branches.iter().peekable();
        while let Some(branch_name) = iter.next() {
            match Branch::from_directory(directory, branch_name).with_context(|| {
                format!(
                    "Failed to get branch {} in directory {}",
                    branch_name,
                    directory.display()
                )
            }) {
                Ok(branch) => println!("{}", branch),
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
            };

            // Print new line between branches, but not at the very end.
            if iter.peek().is_some() {
                println!();
            }
        }

        Ok(())
    }
}

impl AddBranchesArgs {
    pub fn run(&self, modpack: &mut Modpack, _: &ConfigArgs) -> Result<()> {
        let mut iter = self.branches.iter().peekable();
        while let Some(branch_name) = iter.next() {
            let new_branch = modpack.new_branch(branch_name)?;
            println!("{}", &new_branch);

            // Print new line between branches, but not at the very end.
            if iter.peek().is_some() {
                println!();
            }
        }

        Ok(())
    }
}

impl RemoveBranchesArgs {
    pub fn run(&self, modpack: &mut Modpack, _: &ConfigArgs) -> Result<()> {
        println!(
            "These branches in directory {} will be removed:",
            modpack.directory.display()
        );
        for branch in &self.branches {
            println!("  - {}", branch);
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
