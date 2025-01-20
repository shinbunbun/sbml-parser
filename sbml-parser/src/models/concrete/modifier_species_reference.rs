use serde::{Deserialize, Serialize};

use crate::models::{
    primitive::SIdRef,
    r#abstract::{
        s_base::SBaseAttributes, simple_species_reference::SimpleSpeciesReferenceAttributes,
    },
};

// Section 4.11.4
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct ModifierSpeciesReference {
    #[serde(rename = "@species")]
    species: SIdRef,
}

impl SBaseAttributes for ModifierSpeciesReference {}

impl SimpleSpeciesReferenceAttributes for ModifierSpeciesReference {
    fn get_species(&self) -> &String {
        &self.species
    }

    fn set_species(&mut self, species: String) {
        self.species = species;
    }
}
