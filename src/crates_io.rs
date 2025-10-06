//! Structs for communicating with the `crates.io` API.

use crate::{PackrinthError, request_text};
use serde::{Deserialize, Serialize};

/// Struct representative of all versions of a crate on the `crates.io` API.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CratesIoVersions {
    pub versions: Vec<CratesIoVersion>,
}

/// Struct representative of a version on the `crates.io` API.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CratesIoVersion {
    /// The version number of the crate version.
    pub num: String,
}

impl CratesIoVersions {
    /// Gets `crates.io` versions from a crate name.
    ///
    /// # Errors
    /// - [`PackrinthError::FailedToParseCratesIoResponseJson`] if the response was invalid
    pub fn from_crate(crate_name: &str) -> Result<Self, PackrinthError> {
        let endpoint = format!("/crates/{crate_name}/versions");
        let full_url = format!("https://crates.io/api/v1/{endpoint}");
        let crates_io_response = request_text(&full_url)?;

        match serde_json::from_str::<Self>(&crates_io_response) {
            Ok(versions) => Ok(versions),
            Err(error) => Err(PackrinthError::FailedToParseCratesIoResponseJson {
                crates_io_endpoint: endpoint.to_string(),
                error_message: error.to_string(),
            }),
        }
    }
}

/// Checks if a new Packrinth version is available by checking if a newer semantic version is
/// present on `crates.io`.
///
/// If the request was successful and a new version is available, a [`Some`] value with the new version
/// gets returned. If no new version was found, a [`None`] value is the result.
///
/// # Errors
/// - [`PackrinthError::FailedToParseSemverVersion`] if parsing a version to a semver version failed
pub fn is_new_version_available() -> Result<Option<String>, PackrinthError> {
    let newest_version = &CratesIoVersions::from_crate(env!("CARGO_PKG_NAME"))?.versions[0].num;
    let newest_version = match semver::Version::parse(newest_version) {
        Ok(version) => version,
        Err(error) => {
            return Err(PackrinthError::FailedToParseSemverVersion {
                version_to_parse: newest_version.clone(),
                error_message: error.to_string(),
            });
        }
    };
    let current_version = env!("CARGO_PKG_VERSION");
    let current_version = match semver::Version::parse(current_version) {
        Ok(version) => version,
        Err(error) => {
            return Err(PackrinthError::FailedToParseSemverVersion {
                version_to_parse: current_version.to_string(),
                error_message: error.to_string(),
            });
        }
    };

    if newest_version > current_version {
        Ok(Some(newest_version.to_string()))
    } else {
        Ok(None)
    }
}
