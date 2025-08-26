#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::print_stdout,
    clippy::arithmetic_side_effects,
    clippy::as_conversions,
    clippy::integer_division
)]
mod subcommand;

use anyhow::{Result, bail};
use clap::Parser;
use packrinth::config::{self, Modpack};
use std::path::PathBuf;

fn main() -> Result<()> {
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

    /// Export a branch to a Modrinth modpack
    Export(subcommand::ExportArgs),
}

#[derive(Parser, Debug)]
struct ConfigArgs {
    /// Initialize a new modpack if one doesn't exist
    #[clap(long, global = true)]
    pub init: bool,

    /// Set the root directory of the modpack
    #[clap(short, long, global = true)]
    pub directory: Option<PathBuf>,

    /// Output more information about the current process
    #[clap(short, long, global = true)]
    pub verbose: bool,
}

impl Cli {
    fn run(&mut self) -> Result<()> {
        let working_dir = match &self.config_args.directory {
            Some(dir) => dir,
            None => &std::env::current_dir()?,
        };

        let mut modpack = if self.config_args.init {
            Modpack::new(working_dir)?
        } else {
            Modpack::from_directory(working_dir)?
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
            SubCommand::Export(args) => args.run(modpack, config_args),
        }
    }
}
