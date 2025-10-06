#![warn(clippy::pedantic)]
mod cli;
mod subcommand;

use crate::cli::Cli;
use clap::Parser;
use console::Style;
use std::fmt::Display;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
pub const REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");

fn main() {
    // TODO packrinth deploy: deploy to modrinth and GitHub releases
    // TODO add override for env
    // TODO when updating, dependencies look like fj3k29fj, not like fabric-api
    // TODO add launch subcommand to launch a branch
    Cli::parse().run();
}

// All tuples are being made on-the-fly, so adding & would just add more unnecessary syntax
#[allow(clippy::needless_pass_by_value)]
pub fn print_error<T: Display, U: Display>(error: (T, U)) {
    const ERROR_STYLE: Style = Style::new().bold().red();
    const TIP_STYLE: Style = Style::new().green();

    eprintln!("{} {}", ERROR_STYLE.apply_to("error:"), error.0);
    eprintln!();
    eprintln!("  {} {}", TIP_STYLE.apply_to("tip:"), error.1);
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

    println!("{} {}", SUCCESS_STYLE.apply_to("success:"), message);
}
