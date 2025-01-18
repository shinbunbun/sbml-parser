// p14/Section3.2

use serde::{Deserialize, Serialize};

use crate::models::{
    concrete::{annotation::Annotation, notes::Notes},
    primitive::{SBOTerm, SId, ID},
};

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct SBase {
    pub id: Option<SId>,
    pub name: Option<String>,
    pub metaid: Option<ID>,
    #[serde(rename = "sboTerm")]
    pub sbo_term: Option<SBOTerm>,
    pub notes: Option<Notes>,
    pub annotation: Option<Annotation>,
}
