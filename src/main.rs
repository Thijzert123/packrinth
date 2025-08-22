mod config;
mod modrinth;
mod request;
mod subcommand;

use crate::config::Modpack;
use anyhow::{Result, bail};
use clap::Parser;
use std::path::PathBuf;

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
    /// Add or remove Modrinth projects and tweak them for your branches
    Project(subcommand::ProjectArgs),

    /// Create and remove branches that separate your Modpack for various versions
    Branch(subcommand::BranchArgs),

    /// Update branches with the newest project versions
    Update(subcommand::UpdateArgs),
}

#[derive(Parser, Debug)]
struct ConfigArgs {
    /// Initialize a new modpack if one doesn't exist
    #[clap(long, global = true)]
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

        let mut modpack = match self.config_args.init {
            true => Modpack::new(working_dir)?,
            false => Modpack::from_directory(working_dir)?,
        };

        if modpack.pack_format != config::CURRENT_PACK_FORMAT {
            bail!(
                "Pack format {} is not supported by this Packrinth version. Please use a configuration with pack format {}.",
                modpack.pack_format,
                config::CURRENT_PACK_FORMAT
            );
        }

        if let Some(command) = &self.subcommand {
            return command.run(&mut modpack, &self.config_args);
        }

        Ok(())
    }
}

impl SubCommand {
    fn run(&self, modpack: &mut Modpack, config_args: &ConfigArgs) -> Result<()> {
        match self {
            SubCommand::Project(args) => args.run(modpack, config_args),
            SubCommand::Branch(args) => args.run(modpack, config_args),
            SubCommand::Update(args) => args.run(modpack, config_args),
        }
    }
}
