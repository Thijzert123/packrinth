#![warn(clippy::pedantic)]
mod subcommand;

use std::fmt::Display;
use clap::Parser;
use packrinth::config::{self, Modpack};
use std::path::PathBuf;
use console::Style;

fn main() {
    Cli::parse().run();
}

// All tuples are being made on-the-fly, so adding & would just add more unnecessary syntax
#[allow(clippy::needless_pass_by_value)]
pub fn print_error<T: Display, U: Display>(error: (T, U)) {
    const ERROR_STYLE: Style = Style::new().bold().red();
    const TIP_STYLE: Style = Style::new().green();

    eprintln!("{}: {}", ERROR_STYLE.apply_to("error"), error.0);
    eprintln!();
    eprintln!("  {}: {}", TIP_STYLE.apply_to("tip"), error.1);
}

// All tuples are being made on-the-fly, so adding & would just add more unnecessary syntax
#[allow(clippy::needless_pass_by_value)]
pub fn single_line_error<T: ToString, U: ToString>(error: (T, U)) -> String {
    let mut single_line_error = error.0.to_string();
    single_line_error.push_str(": ");
    single_line_error.push_str(&error.1.to_string());
    single_line_error
}

pub fn print_success<T: Display>(message: T) {
    const SUCCESS_STYLE: Style = Style::new().bold().green();

    println!("{}: {}", SUCCESS_STYLE.apply_to("success"), message);
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
    fn run(&mut self) {
        self.subcommand.run(&self.config_args);
    }
}

impl SubCommand {
    fn run(&self, config_args: &ConfigArgs) {
        let current_dir = match &config_args.directory {
            Some(dir) => dir,
            None => match std::env::current_dir() {
                Ok(current_dir) => &current_dir.clone(),
                Err(_error) => {
                    print_error(("couldn't get current directory", "the current directory may not exist or you have insufficient permissions to access the current directory"));
                    return;
                },
            },
        };

        if let Self::Init = self {
            let modpack = match Modpack::new(current_dir) {
                Ok(modpack) => modpack,
                Err(error) => {
                    print_error(error.message_and_tip());
                    return;
                }
            };

            match modpack.save() {
                Ok(()) => print_success(format!("created new modpack instance in directory {}", current_dir.display())),
                Err(error) => print_error(error.message_and_tip()),
            }

            return;
        }

        let mut modpack = match Modpack::from_directory(current_dir) {
            Ok(modpack) => modpack,
            Err(error) => {
                print_error(error.message_and_tip());
                return;
            }
        };

        if modpack.pack_format != config::CURRENT_PACK_FORMAT {
            print_error((format!("pack format {} is not supported by this Packrinth version", modpack.pack_format), format!("please use a configuration with pack format {}", config::CURRENT_PACK_FORMAT)));
            return;
        }

        match self {
            SubCommand::Init => (),
            SubCommand::Project(args) => args.run(&mut modpack, config_args),
            SubCommand::Branch(args) => args.run(&mut modpack, config_args),
            SubCommand::Update(args) => args.run(&modpack, config_args),
            SubCommand::Export(args) => args.run(&mut modpack, config_args),
        }
    }
}
