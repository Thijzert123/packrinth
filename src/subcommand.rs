use crate::ConfigArgs;
use crate::json::config::{Branch, Modpack, newest_version_for_project};
use clap::{Parser, Subcommand};
use std::fmt::Display;

#[derive(Debug, Parser)]
pub struct BranchArgs {
    branch: String,
}

#[derive(Debug, Parser)]
pub struct UpdateArgs {
    branch: Option<String>,
}

impl UpdateArgs {
    pub fn run(
        &self,
        modpack: Modpack,
        config_args: &ConfigArgs,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let None = self.branch {
            for project in modpack.projects {
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
        modpack: Modpack,
        config_args: &ConfigArgs,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let branch = Branch::from_working_dir(modpack, &self.branch, config_args.new)?;

        println!("Branch {}:", &self.branch);
        println!("  - Pack version: {}", &branch.version);
        println!(
            "  - Minecraft versions: {}",
            &branch.minecraft_versions.join(", ")
        );
        println!("  - Loaders: {:?}", &branch.loaders); // TODO make this pretty

        Ok(())
    }
}
