// p14/Section3.2

use serde::{Deserialize, Serialize};

use crate::models::{
    concrete::{annotation::Annotation, notes::Notes},
    primitive::{SBOTerm, SId, ID},
};

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
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
}
