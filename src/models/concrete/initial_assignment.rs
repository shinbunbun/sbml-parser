use serde::{Deserialize, Serialize};

use crate::models::primitive::{SBOTerm, SId, SIdRef};

use super::math::Math;

// Section4.8
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct InitialAssignment {
    pub id: Option<SId>,
    pub symbol: SIdRef,
    pub math: Option<Math>,
    #[serde(rename = "sboTerm")]
    pub sbo_term: Option<SBOTerm>,
}
