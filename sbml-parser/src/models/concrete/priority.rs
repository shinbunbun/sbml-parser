use serde::{Deserialize, Serialize};

use super::math::Math;
use crate::models::{primitive::ID, r#abstract::s_base::SBaseAttributes};

// p79/Section4.12.3
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct Priority {
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ID>,
    #[serde(rename = "@sboTerm", skip_serializing_if = "Option::is_none")]
    pub sbo_term: Option<ID>,
    #[serde(rename = "@math", skip_serializing_if = "Option::is_none")]
    pub math: Option<Math>,
}

impl SBaseAttributes for Priority {
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
