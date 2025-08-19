mod json;
mod request;
mod subcommand;

use std::path::{Path, PathBuf};
use anyhow::{bail, Result};
use crate::json::config;
use crate::json::config::Modpack;
use clap::Parser;

fn main() -> Result<()> {
    // Initialize logger so that the user sees the logs in the terminal
    simple_logger::init_with_level(log::Level::Debug).unwrap_or_else(|error| {
        eprintln!("Could not initialize logger: {}", error);
    });

    Cli::parse().run()
}

#[derive(Parser, Debug)]
struct Cli {
    #[clap(subcommand)]
    subcommand: Option<SubCommand>,

    #[clap(flatten)]
    config_args: ConfigArgs,
}

#[derive(Parser, Debug)]
enum SubCommand {
    Branch(subcommand::BranchArgs),
    Update(subcommand::UpdateArgs),
}

#[derive(Parser, Debug)]
struct ConfigArgs {
    /// Initialize a new modpack if one doesn't exist
    #[clap(short, long, global = true)]
    pub init: bool,

    /// Set the root directory of the modpack
    #[clap(short, long, global = true)]
    pub directory: Option<PathBuf>,
}

impl Cli {
    fn run(&mut self) -> Result<()> {
        let working_dir = match &self.config_args.directory {
            Some(dir) => dir,
            None => &std::env::current_dir()?,
        };

        let mut modpack =  match self.config_args.init {
            true => Modpack::new(working_dir)?,
            false => Modpack::from_directory(working_dir)?,
        };

        if modpack.pack_format != config::CURRENT_PACK_FORMAT {
            bail!("Pack format {} is not supported by this Packrinth version. Please use a configuration with pack format {}.", modpack.pack_format, config::CURRENT_PACK_FORMAT);
        }

        if let Some(command) = &self.subcommand {
            return command.run(working_dir, &mut modpack, &self.config_args);
        }

        Ok(())
    }
}

impl SubCommand {
    fn run(
        &self,
        directory: &Path,
        modpack: &mut Modpack,
        config_args: &ConfigArgs,
    ) -> Result<()> {
        match self {
            SubCommand::Branch(args) => args.run(directory, modpack, config_args),
            SubCommand::Update(args) => args.run(directory, modpack, config_args),
        }
    }
}