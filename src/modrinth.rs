//! Structs that are only used for deserializing JSON responses directly from the Modrinth API.

use crate::config::Side;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub title: String,
    pub server_side: SideSupport,
    pub client_side: SideSupport,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SideSupport {
    #[serde(rename = "required")]
    Required,

    #[serde(rename = "optional")]
    Optional,

    #[serde(rename = "unsupported")]
    Unsupported,
}

/// Struct that can be used to deserialize Modrinth version JSONs (/version).
#[derive(Debug, Serialize, Deserialize)]
pub struct Version {
    pub id: String,
    pub project_id: String,
    pub version_number: String,
    pub files: Vec<VersionFile>,
}

/// File in a version.
#[derive(Debug, Serialize, Deserialize)]
pub struct VersionFile {
    pub url: String,
    pub filename: String,
    pub primary: bool,
    pub size: u64,
    pub hashes: FileHashes,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileHashes {
    pub sha512: String,
}

impl Project {
    pub fn side(&self) -> Side {
        if let SideSupport::Required = self.server_side
            && let SideSupport::Unsupported = self.client_side
        {
            Side::Server
        } else if let SideSupport::Unsupported = self.server_side
            && let SideSupport::Required = self.client_side
        {
            Side::Client
        } else {
            Side::Both
        }
    }
}
