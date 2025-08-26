#![warn(clippy::pedantic)]
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
    subcommand: SubCommand,

    #[clap(flatten)]
    config_args: ConfigArgs,
}

#[derive(Parser, Debug)]
enum SubCommand {
    /// Initialize a new modpack project
    Init,

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
    /// Set the root directory of the modpack
    #[clap(short, long, global = true)]
    pub directory: Option<PathBuf>,

    /// Output more information about the current process
    #[clap(short, long, global = true)]
    pub verbose: bool,
}

impl Cli {
    fn run(&mut self) -> Result<()> {
        self.subcommand.run(&self.config_args)
    }
}

impl SubCommand {
    fn run(&self, config_args: &ConfigArgs) -> Result<()> {
        let working_dir = match &config_args.directory {
            Some(dir) => dir,
            None => &std::env::current_dir()?,
        };

        let mut modpack = if let Self::Init = self {
            let modpack = Modpack::new(working_dir)?;
            println!("Created new modpack instance in {}", working_dir.display());
            modpack
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

        match self {
            SubCommand::Init => Ok(()),
            SubCommand::Project(args) => args.run(&mut modpack, config_args),
            SubCommand::Branch(args) => args.run(&mut modpack, config_args),
            SubCommand::Update(args) => args.run(&modpack, config_args),
            SubCommand::Export(args) => args.run(&mut modpack, config_args),
        }
    }
}
