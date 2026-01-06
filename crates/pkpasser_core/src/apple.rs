//! # Utilities for .pkpass generation
//!
//! Provides structs and utilities for boilerplating and signing .pkpass files

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

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

pub struct Manifest(HashMap<String, String>);

impl Manifest {
    pub fn from_dir() {}
}
