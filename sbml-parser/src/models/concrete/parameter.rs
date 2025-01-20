use serde::{Deserialize, Serialize};

use crate::models::{
    primitive::{SBOTerm, UnitSidRef, ID},
    r#abstract::s_base::SBaseAttributes,
};

// p53/Section4.7
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct Parameter {
    #[serde(rename = "@id")]
    pub id: ID,
    #[serde(rename = "@sboTerm", skip_serializing_if = "Option::is_none")]
    pub sbo_term: Option<SBOTerm>,
    #[serde(rename = "@value", skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
    #[serde(rename = "@units", skip_serializing_if = "Option::is_none")]
    pub units: Option<UnitSidRef>,
    #[serde(rename = "@constant")]
    pub constant: bool,
}

impl SBaseAttributes for Parameter {
    fn get_id(&self) -> Option<&String> {
        Some(&self.id)
    }

    fn set_id(&mut self, id: String) {
        self.id = id;
    }

    fn get_sbo_term(&self) -> Option<&String> {
        self.sbo_term.as_ref()
    }

    fn set_sbo_term(&mut self, sbo_term: String) {
        self.sbo_term = Some(sbo_term);
    }
}
