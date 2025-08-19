//! This module provides structs that can be used for (de)serializing JSON data.

pub mod config;
mod modrinth;

use std::fmt::Debug;
use anyhow::{Context, Result};
use serde::Serialize;
use std::fs;
use std::path::Path;

fn json_to_file<T, P>(json_value: &T, file: P) -> Result<()>
where
    T: ?Sized + Serialize + Debug,
    P: AsRef<Path>,
{
    let json = serde_json::to_string_pretty(json_value).with_context(|| format!("Failed to serialize {:?} to JSON", json_value))?;
    fs::write(&file, json).with_context(|| format!("Failed write to {}", &file.as_ref().display()))?;
    Ok(())
}
