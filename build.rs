use crate::cli::Cli;
use std::path::PathBuf;
use std::{env, fs};

#[path = "src/cli.rs"]
mod cli;

fn main() {
    // Generate CLI Markdown documentation and write it to a file
    let doc = clap_markdown::help_markdown_custom::<Cli>(
        &clap_markdown::MarkdownOptions::new().show_footer(false),
    );
    let doc_file = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("CLI_HELP.md");
    fs::write(&doc_file, doc).unwrap();
}
