mod json;
mod request;
mod subcommand;

use crate::json::config;
use crate::json::config::Modpack;
use clap::Parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logger so that the user sees the logs in the terminal
    simple_logger::init_with_level(log::Level::Debug).unwrap_or_else(|error| {
        println!("Could not initialize logger: {}", error);
    });

    // println!("{:#?}", Version::from_id("ND4ROcMQ")?);
    //
    // println!("{:#?}", Modpack::from_working_dir()?);

    Cli::parse().run()?;

    Ok(())
}

#[derive(Parser, Debug)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Command>,

    #[clap(flatten)]
    config_args: ConfigArgs,
}

#[derive(Parser, Debug)]
enum Command {
    Branch(subcommand::BranchArgs),
    Update(subcommand::UpdateArgs),
}

#[derive(Parser, Debug)]
struct ConfigArgs {
    /// If no modpack configuration file exists, initialize a new project.
    /// When this option is passed, a new branch might also be created.
    #[clap(short, long, global = true)]
    new: bool,
}

impl Cli {
    fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let modpack = Modpack::from_working_dir(self.config_args.new)?;
        if modpack.pack_format != config::CURRENT_PACK_FORMAT {
            // TODO add better handling to user here
            println!("Current pack format is not supported");
            return Ok(());
        }

        if let Some(command) = &self.command {
            return command.run(modpack, &self.config_args);
        }

        Ok(())
    }
}

impl Command {
    fn run(
        &self,
        modpack: Modpack,
        config_args: &ConfigArgs,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Command::Branch(args) => args.run(modpack, config_args),
            Command::Update(args) => args.run(modpack, config_args),
        }
    }
}
