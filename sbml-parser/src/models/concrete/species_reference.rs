use serde::{Deserialize, Serialize};

use super::simple_species_reference::SimpleSpeciesReference;

// Section 4.11.3
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct SpeciesReference {
    #[serde(rename = "@stoichiometry" ,skip_serializing_if = "Option::is_none")]
    pub stoichiometry: Option<f64>,
    #[serde(rename = "@denominator" )]
    pub constant: bool,
    #[serde(rename = "@simpleSpeciesReference", flatten)]
    pub simple_species_reference: SimpleSpeciesReference,
}
