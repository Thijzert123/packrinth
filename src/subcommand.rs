use std::path::Path;
use crate::ConfigArgs;
use crate::json::config::{Branch, Modpack};
use clap::{Parser};
use anyhow::Result;

#[derive(Debug, Parser)]
pub struct BranchArgs {
    #[clap(subcommand)]
    command: Option<BranchSubCommand>,

    branches: Option<Vec<String>>,
}

#[derive(Parser, Debug)]
enum BranchSubCommand {
    List(ListBranchesArgs),
    Add(AddBranchArgs),
}

#[derive(Parser, Debug)]
struct ListBranchesArgs;

#[derive(Parser, Debug)]
struct AddBranchArgs {
    branches: Vec<String>,
}

#[derive(Debug, Parser)]
pub struct UpdateArgs {
    branch: Option<String>,
}

impl UpdateArgs {
    pub fn run(
        &self,
        directory: &Path,
        modpack: &Modpack,
        config_args: &ConfigArgs,
    ) -> Result<()> {
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
                BranchSubCommand::Add(args) => args.run(directory, modpack, config_args)
            }
        } else if let Some(branch_names) = &self.branches {
            let mut iter = branch_names.iter().peekable();
            while let Some(branch_name) = iter.next() {
                let branch = Branch::from_directory(directory, branch_name)?;
                println!("{}", &branch);

                // Print new line between branches, but not at the very end.
                if iter.peek().is_some() {
                    println!();
                }
            }

            Ok(())
        } else {
            ListBranchesArgs::run(&ListBranchesArgs {}, directory, modpack, config_args)
        }
    }
}

impl ListBranchesArgs {
    pub fn run(&self, directory: &Path, modpack: &Modpack, config_args: &ConfigArgs) -> Result<()> {
        // TODO add this command
        println!("Listing branches...");
        Ok(())
    }
}

impl AddBranchArgs {
    pub fn run(&self, directory: &Path, modpack: &mut Modpack, config_args: &ConfigArgs) -> Result<()> {
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