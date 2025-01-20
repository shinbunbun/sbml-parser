use serde::{Deserialize, Serialize};

use crate::models::{
    primitive::{LocalSId, SBOTerm, UnitSidRef},
    r#abstract::s_base::SBaseAttributes,
};

// Section 4.11.6
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct LocalParameter {
    pub id: LocalSId,
    #[serde(rename = "@sboTerm", skip_serializing_if = "Option::is_none")]
    pub sbo_term: Option<SBOTerm>,
    #[serde(rename = "@value", skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
    #[serde(rename = "@units", skip_serializing_if = "Option::is_none")]
    pub units: Option<UnitSidRef>,
}

impl SBaseAttributes for LocalParameter {
    fn get_id(&self) -> Option<&LocalSId> {
        Some(&self.id)
    }

    fn set_id(&mut self, id: LocalSId) {
        self.id = id;
    }

    fn get_sbo_term(&self) -> Option<&String> {
        self.sbo_term.as_ref()
    }

    fn set_sbo_term(&mut self, sbo_term: String) {
        self.sbo_term = Some(sbo_term);
    }
}
