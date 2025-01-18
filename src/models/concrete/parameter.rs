use serde::{Deserialize, Serialize};

use crate::models::primitive::{SBOTerm, SId, UnitSidRef};

// p53/Section4.7
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct Parameter {
    pub id: SId,
    pub value: Option<f64>,
    pub units: Option<UnitSidRef>,
    pub constant: bool,
    #[serde(rename = "sboTerm")]
    pub sbo_term: Option<SBOTerm>,
}
