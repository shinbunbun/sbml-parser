use serde::{Deserialize, Serialize};

use crate::models::{primitive::SIdRef, r#abstract::s_base::SBase};

use super::{
    kinetic_law::KineticLaw, modifier_species_reference::ModifierSpeciesReference,
    species_reference::SpeciesReference,
};

// Section 4.11
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct Reaction {
    pub reversible: bool,
    pub compartment: Option<SIdRef>,
    pub list_of_reactants: Vec<ListOfSpeciesReference>,
    pub list_of_products: Vec<ListOfSpeciesReference>,
    pub list_of_modifiers: Vec<ListOfModifierSpeciesReferences>,
    pub kinetic_law: Option<KineticLaw>,
    #[serde(flatten)]
    pub s_base: SBase, // todo: pub id: String
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct ListOfSpeciesReference {
    pub species_reference: Vec<SpeciesReference>,
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct ListOfModifierSpeciesReferences {
    pub modifier_species_reference: Vec<ModifierSpeciesReference>,
    #[serde(flatten)]
    pub s_base: SBase,
}
