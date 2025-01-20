use serde::{Deserialize, Serialize};

use crate::models::{
    primitive::{SBOTerm, SId},
    r#abstract::s_base::SBaseAttributes,
};

use super::math::Math;

// Section 4.3
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct FunctionDefinition {
    #[serde(rename = "@id")]
    pub id: SId,
    #[serde(rename = "@sboTerm", skip_serializing_if = "Option::is_none")]
    pub sbo_term: Option<SBOTerm>,
    #[serde(rename = "@math", skip_serializing_if = "Option::is_none")]
    pub math: Option<Math>,
}

impl SBaseAttributes for FunctionDefinition {
    fn get_id(&self) -> Option<&SId> {
        Some(&self.id)
    }

    fn set_id(&mut self, id: SId) {
        self.id = id;
    }

    fn get_sbo_term(&self) -> Option<&String> {
        self.sbo_term.as_ref()
    }

    fn set_sbo_term(&mut self, sbo_term: String) {
        self.sbo_term = Some(sbo_term);
    }
}
