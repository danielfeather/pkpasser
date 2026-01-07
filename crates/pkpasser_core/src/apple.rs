//! # Utilities for .pkpass generation
//!
//! Provides structs and utilities for boilerplating and signing .pkpass files

use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fmt::Write,
    fs,
    io::Read,
    path::{Path, PathBuf},
};
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PassType {
    /// An object that represents the groups of fields that display the information for a boarding pass
    BoardingPass(BoardingPass),
    Coupon(PassFields),
    EventTicket(PassFields),
    Generic(PassFields),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BoardingPass {
    transit_type: TransitType,
    #[serde(flatten)]
    pass_fields: PassFields,
}

/// The type of transit for a boarding pass
#[derive(Debug, Deserialize, Serialize)]
pub enum TransitType {
    #[serde(rename = "PKTransitTypeAir")]
    Air,
    #[serde(rename = "PKTransitTypeBoat")]
    Boat,
    #[serde(rename = "PKTransitTypeBus")]
    Bus,
    #[serde(rename = "PKTransitTypeGeneric")]
    Generic,
    #[serde(rename = "PKTransitTypeTrain")]
    Train,
}

#[derive(Debug, Serialize, Deserialize)]
struct AppStoreId(u64);

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pass {
    #[serde(rename = "accessibilityURL", skip_serializing_if = "Option::is_none")]
    accessibility_url: Option<String>,
    #[serde(rename = "addOnURL", skip_serializing_if = "Option::is_none")]
    add_on_url: Option<String>,
    #[serde(rename = "appLaunchURL", skip_serializing_if = "Option::is_none")]
    app_launch_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    associated_store_identifiers: Option<Vec<AppStoreId>>,
    #[serde(flatten)]
    pass_type: PassType,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PassFields {
    additional_info_fields: PassFieldContent,
    auxiliary_fields: AuxiliaryFieldContent,
    back_fields: PassFieldContent,
    header_fields: PassFieldContent,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuxiliaryFieldContent {
    row: u64,
    #[serde(flatten)]
    field_content: PassFieldContent,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PassFieldContent {}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Manifest(HashMap<PathBuf, String>);

#[derive(Debug, Error)]
#[error("CreateManifestError")]
pub enum CreateManifestError {
    NotADirectory,
}

impl Manifest {
    /// Create a manifest from a directory by hashing all files
    /// recursively in a directory
    pub fn from_dir(dir: &Path) -> Result<Self, anyhow::Error> {
        if !dir.is_dir() {
            return Err(CreateManifestError::NotADirectory.into());
        }

        let mut hash_table = HashMap::new();

        for entry in fs::read_dir(dir).map_err(|e| e)? {
            let entry = entry?;

            let path = entry.path();

            if !path.is_dir() {
                let mut content = std::fs::File::open(path)?;

                let mut buf = Vec::new();

                content.read_to_end(&mut buf)?;

                let sha = openssl::sha::sha1(&buf);

                let mut hash = String::with_capacity(20);

                for byte in sha {
                    write!(&mut hash, "{byte:x}")?;
                }

                hash_table.insert(entry.path(), hash);

                continue;
            }

            hash_table.insert(entry.path(), String::new());
        }

        Ok(Self(hash_table))
    }
}
