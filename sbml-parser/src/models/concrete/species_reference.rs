use serde::{Deserialize, Serialize};

use crate::models::{
    primitive::{SIdRef, ID},
    r#abstract::{
        s_base::SBaseAttributes, simple_species_reference::SimpleSpeciesReferenceAttributes,
    },
};

// Section 4.11.3
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct SpeciesReference {
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ID>,
    #[serde(rename = "@species")]
    pub species: SIdRef,
    #[serde(rename = "@stoichiometry", skip_serializing_if = "Option::is_none")]
    pub stoichiometry: Option<f64>,
    #[serde(rename = "@constant")]
    pub constant: bool,
}

impl SBaseAttributes for SpeciesReference {
    fn get_id(&self) -> Option<&String> {
        self.id.as_ref()
    }

    fn set_id(&mut self, id: String) {
        self.id = Some(id);
    }
}

impl SimpleSpeciesReferenceAttributes for SpeciesReference {
    fn get_species(&self) -> &String {
        &self.species
    }

    fn set_species(&mut self, species: String) {
        self.species = species;
    }
}
