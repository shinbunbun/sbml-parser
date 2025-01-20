use serde::{Deserialize, Serialize};

use crate::models::{
    primitive::{SBOTerm, SIdRef},
    r#abstract::s_base::SBaseAttributes,
};

// Section 4.11.2
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct SimpleSpeciesReference {
    #[serde(rename = "@sboTerm", skip_serializing_if = "Option::is_none")]
    pub sbo_term: Option<SBOTerm>,
    #[serde(rename = "@species")]
    pub species: SIdRef,
}

impl SBaseAttributes for SimpleSpeciesReference {
    fn get_sbo_term(&self) -> Option<&String> {
        self.sbo_term.as_ref()
    }

    fn set_sbo_term(&mut self, sbo_term: String) {
        self.sbo_term = Some(sbo_term);
    }
}
