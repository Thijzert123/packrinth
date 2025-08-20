use crate::ConfigArgs;
use crate::json::config;
use crate::json::config::{Branch, Modpack};
use anyhow::{Context, Result, bail};
use clap::Parser;
use std::path::Path;

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

#[derive(Debug, Parser)]
pub struct UpdateArgs {
    branch: Option<String>,
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