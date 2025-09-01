use std::io::Write;
use crate::cli::Cli;
use std::path::PathBuf;
use std::env;
use std::fs::OpenOptions;

#[path = "src/cli.rs"]
mod cli;

fn main() {
    // Generate CLI Markdown documentation and write it to a file
    let doc = clap_markdown::help_markdown_custom::<Cli>(
        &clap_markdown::MarkdownOptions::new().show_footer(false),
    );

    let doc_file_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("docs").join("cli-help.md");

    let mut doc_file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(&doc_file_path).unwrap();

    writeln!(doc_file, "---").unwrap();

    let mut doc_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&doc_file_path).unwrap();

    writeln!(doc_file, "title: CLI Help").unwrap();
    writeln!(doc_file, "layout: default").unwrap();
    writeln!(doc_file, "---").unwrap();
    writeln!(doc_file).unwrap();
    writeln!(doc_file, "{}", doc).unwrap();
}
