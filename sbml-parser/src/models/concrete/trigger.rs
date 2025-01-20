use serde::{Deserialize, Serialize};

use super::math::Math;
use crate::models::{
    primitive::{SBOTerm, SId},
    r#abstract::s_base::SBaseAttributes,
};

// p79/Section4.12.2
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct Trigger {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<SId>,
    #[serde(rename = "sboTerm", skip_serializing_if = "Option::is_none")]
    pub sbo_term: Option<SBOTerm>,
    #[serde(rename = "ititialValue")]
    pub ititial_value: bool,
    pub persistent: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub math: Option<Math>,
}

impl SBaseAttributes for Trigger {
    fn get_id(&self) -> Option<&SId> {
        self.id.as_ref()
    }

    fn set_id(&mut self, id: SId) {
        self.id = Some(id);
    }

    fn get_sbo_term(&self) -> Option<&String> {
        self.sbo_term.as_ref()
    }

    fn set_sbo_term(&mut self, sbo_term: String) {
        self.sbo_term = Some(sbo_term);
    }
}
