use serde::{Deserialize, Serialize};

use super::simple_species_reference::SimpleSpeciesReference;

// Section 4.11.4
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct ModifierSpeciesReference {
    #[serde(flatten)]
    pub simple_species_reference: SimpleSpeciesReference,
}
