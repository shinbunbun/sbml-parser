// p14/Section3.2

use crate::models::{
    concrete::{annotation::Annotation, notes::Notes},
    primitive::{SBOTerm, SId, ID},
};

/* #[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct SBase {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<SId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metaid: Option<ID>,
    #[serde(rename = "sboTerm", skip_serializing_if = "Option::is_none")]
    pub sbo_term: Option<SBOTerm>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Notes>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotation: Option<Annotation>,
} */

pub trait SBaseAttributes {
    fn get_id(&self) -> Option<&SId> {
        None
    }
    fn set_id(&mut self, _id: SId) {}

    fn get_name(&self) -> Option<&String> {
        None
    }
    fn set_name(&mut self, _name: String) {}

    fn get_metaid(&self) -> Option<&ID> {
        None
    }
    fn set_metaid(&mut self, _metaid: ID) {}

    fn get_sbo_term(&self) -> Option<&SBOTerm> {
        None
    }
    fn set_sbo_term(&mut self, _sbo_term: SBOTerm) {}

    fn get_notes(&self) -> Option<&Notes> {
        None
    }
    fn set_notes(&mut self, _notes: Notes) {}

    fn get_annotation(&self) -> Option<&Annotation> {
        None
    }
    fn set_annotation(&mut self, _annotation: Annotation) {}
}
