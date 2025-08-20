use std::collections::HashMap;
use crate::ConfigArgs;
use crate::json::config;
use crate::json::config::{Branch, IncludeOrExclude, Modpack, ProjectSettings};
use anyhow::{Context, Result, bail};
use clap::Parser;
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

    Add(AddProjectArgs),

    #[clap(alias = "rm")]
    Remove(RemoveProjectArgs),
}

#[derive(Parser, Debug)]
struct ListProjectsArgs;

#[derive(Parser, Debug)]
struct AddProjectArgs {
    projects: Vec<String>,
}

#[derive(Parser, Debug)]
struct RemoveProjectArgs {
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

    Add(AddBranchArgs),

    #[clap(alias = "rm")]
    Remove(RemoveBranchArgs),
}

#[derive(Parser, Debug)]
struct ListBranchesArgs;

#[derive(Parser, Debug)]
struct AddBranchArgs {
    branches: Vec<String>,
}

#[derive(Parser, Debug)]
struct RemoveBranchArgs {
    branches: Vec<String>,
}

impl ProjectArgs {
    pub fn run(
        &self,
        directory: &Path,
        modpack: &mut Modpack,
        config_args: &ConfigArgs,
    ) -> Result<()> {
        if let Some(command) = &self.command {
            match command {
                ProjectSubCommand::List(args) => args.run(directory, modpack, config_args),
                ProjectSubCommand::Add(args) => args.run(directory, modpack, config_args),
                ProjectSubCommand::Remove(args) => args.run(directory, modpack, config_args),
            }
        } else if let Some(project_names) = &self.projects {
            let project_map = project_names.into_iter().map(|x| (x.clone(), None)).collect();
            ListProjectsArgs::list(&project_map)
        } else {
            ListProjectsArgs::run(&ListProjectsArgs {}, directory, modpack, config_args)
        }
    }
}

impl ListProjectsArgs {
    pub fn run(&self, _: &Path, modpack: &mut Modpack, _: &ConfigArgs) -> Result<()> {
        Self::list(&modpack.projects)
    }

    pub fn list(projects: &HashMap<String, Option<ProjectSettings>>) -> Result<()> {
        if projects.is_empty() {
            println!("There are no projects added to this modpack yet.");
            return Ok(());
        }

        let mut iter = projects.iter().peekable();
        while let Some(project) = iter.next() {
            println!("{}", project.0);

            if let Some(project_settings) = project.1 {
                if let Some(overrides) = &project_settings.version_overrides {
                    println!("  - Overrides:");
                    for version_override in overrides {
                        println!("    - {}: {}", version_override.0, version_override.1);
                    }
                }

                if let Some(include_or_exclude) = &project_settings.include_or_exclude {
                    match include_or_exclude {
                        IncludeOrExclude::Include(includes) => println!("  - Includes: {}", includes.join(", ")),
                        IncludeOrExclude::Exclude(excludes) => println!("  - Excludes: {}", excludes.join(", ")),
                    }
                }
            }

            // Print new line between projects, but not at the very end.
            if iter.peek().is_some() {
                println!();
            }
        }

        Ok(())
    }
}

impl AddProjectArgs {
    pub fn run(&self, directory: &Path, modpack: &mut Modpack, _: &ConfigArgs) -> Result<()> {
        Ok(())
    }
}

impl RemoveProjectArgs {
    pub fn run(&self, directory: &Path, modpack: &mut Modpack, _: &ConfigArgs) -> Result<()> {
        Ok(())
    }
}

impl UpdateArgs {
    pub fn run(&self, directory: &Path, modpack: &Modpack, config_args: &ConfigArgs) -> Result<()> {
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
    pub fn run(
        &self,
        directory: &Path,
        modpack: &mut Modpack,
        config_args: &ConfigArgs,
    ) -> Result<()> {
        if let Some(command) = &self.command {
            match command {
                BranchSubCommand::List(args) => args.run(directory, modpack, config_args),
                BranchSubCommand::Add(args) => args.run(directory, modpack, config_args),
                BranchSubCommand::Remove(args) => args.run(directory, modpack, config_args),
            }
        } else if let Some(branch_names) = &self.branches {
            ListBranchesArgs::list(directory, branch_names)
        } else {
            ListBranchesArgs::run(&ListBranchesArgs {}, directory, modpack, config_args)
        }
    }
}

impl ListBranchesArgs {
    pub fn run(&self, directory: &Path, modpack: &Modpack, _: &ConfigArgs) -> Result<()> {
        Self::list(directory, &modpack.branches)
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

impl AddBranchArgs {
    pub fn run(&self, directory: &Path, modpack: &mut Modpack, _: &ConfigArgs) -> Result<()> {
        let mut iter = self.branches.iter().peekable();
        while let Some(branch_name) = iter.next() {
            let new_branch = Branch::new(directory, modpack, branch_name)?;
            println!("{}", &new_branch);

            // Print new line between branches, but not at the very end.
            if iter.peek().is_some() {
                println!();
            }
        }

        Ok(())
    }
}

impl RemoveBranchArgs {
    pub fn run(&self, directory: &Path, modpack: &mut Modpack, _: &ConfigArgs) -> Result<()> {
        Branch::remove_all(directory, modpack, &self.branches)
    }
}