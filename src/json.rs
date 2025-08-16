//! This module provides structs that can be used for (de)serializing JSON data.

pub mod config;
mod modrinth;

use anyhow::Result;
use serde::Serialize;
use std::fs;
use std::path::Path;

fn json_to_file<T, P>(json_value: &T, file: P) -> Result<()>
where
    T: ?Sized + Serialize,
    P: AsRef<Path>,
{
    let json = serde_json::to_string_pretty(json_value)?;
    fs::write(file, json)?;
    Ok(())
}
