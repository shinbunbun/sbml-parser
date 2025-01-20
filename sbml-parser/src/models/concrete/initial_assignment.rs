use serde::{Deserialize, Serialize};

use crate::models::{
    primitive::{SBOTerm, SIdRef, ID},
    r#abstract::s_base::SBaseAttributes,
};

use super::math::Math;

// Section4.8
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct InitialAssignment {
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ID>,
    #[serde(rename = "@sboTerm", skip_serializing_if = "Option::is_none")]
    pub sbo_term: Option<SBOTerm>,
    #[serde(rename = "@symbol")]
    pub symbol: SIdRef,
    #[serde(rename = "@math")]
    pub math: Option<Math>,
}

impl SBaseAttributes for InitialAssignment {
    fn get_id(&self) -> Option<&String> {
        self.id.as_ref()
    }

    fn set_id(&mut self, id: String) {
        self.id = Some(id);
    }

    fn get_sbo_term(&self) -> Option<&String> {
        self.sbo_term.as_ref()
    }

    fn set_sbo_term(&mut self, sbo_term: String) {
        self.sbo_term = Some(sbo_term);
    }
}
