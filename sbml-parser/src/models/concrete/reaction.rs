use serde::{Deserialize, Serialize};

use crate::models::{
    primitive::{SIdRef, ID},
    r#abstract::s_base::SBaseAttributes,
};

use super::{
    kinetic_law::KineticLaw, modifier_species_reference::ModifierSpeciesReference,
    species_reference::SpeciesReference,
};

// Section 4.11
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct Reaction {
    pub id: ID,
    #[serde(rename = "sboTerm", skip_serializing_if = "Option::is_none")]
    pub sbo_term: Option<String>,
    pub reversible: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compartment: Option<SIdRef>,
    #[serde(rename = "listOfReactants", skip_serializing_if = "Option::is_none")]
    pub list_of_reactants: Option<ListOfSpeciesReference>,
    #[serde(rename = "listOfProducts", skip_serializing_if = "Option::is_none")]
    pub list_of_products: Option<ListOfSpeciesReference>,
    #[serde(rename = "listOfModifiers", skip_serializing_if = "Option::is_none")]
    pub list_of_modifiers: Option<ListOfModifierSpeciesReferences>,
    #[serde(rename = "kineticLaw", skip_serializing_if = "Option::is_none")]
    pub kinetic_law: Option<KineticLaw>,
}

impl SBaseAttributes for Reaction {
    fn get_id(&self) -> Option<&String> {
        Some(&self.id)
    }

    fn set_id(&mut self, id: ID) {
        self.id = id;
    }

    fn get_sbo_term(&self) -> Option<&String> {
        self.sbo_term.as_ref()
    }

    fn set_sbo_term(&mut self, sbo_term: String) {
        self.sbo_term = Some(sbo_term);
    }
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct ListOfSpeciesReference {
    #[serde(rename = "speciesReference", skip_serializing_if = "Option::is_none")]
    pub species_reference: Option<Vec<SpeciesReference>>,
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct ListOfModifierSpeciesReferences {
    #[serde(
        rename = "modifierSpeciesReference",
        skip_serializing_if = "Option::is_none"
    )]
    pub modifier_species_reference: Option<Vec<ModifierSpeciesReference>>,
}

impl SBaseAttributes for ListOfSpeciesReference {}
