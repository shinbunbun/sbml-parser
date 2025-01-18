use serde::{Deserialize, Serialize};

use super::simple_species_reference::SimpleSpeciesReference;

// Section 4.11.3
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct SpeciesReference {
    pub stoichiometry: Option<f64>,
    pub constant: bool,
    #[serde(flatten)]
    pub simple_species_reference: SimpleSpeciesReference,
}
