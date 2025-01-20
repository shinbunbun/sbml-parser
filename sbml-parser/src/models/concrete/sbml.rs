use serde::{Deserialize, Serialize};

use crate::models::{
    primitive::{SBOTerm, ID},
    r#abstract::s_base::SBaseAttributes,
};

use super::model::Model;

// Section4.1
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct SBML {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<ID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "sboTerm", skip_serializing_if = "Option::is_none")]
    pub sbo_term: Option<SBOTerm>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metaid: Option<ID>,
    /// fixed: 3
    pub level: u8,
    /// fixed: 2
    pub version: u8,
    /// fixed: http://www.sbml.org/sbml/level3/version2/core
    pub xmlns: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<Model>,
}

impl SBaseAttributes for SBML {
    fn get_id(&self) -> Option<&String> {
        self.id.as_ref()
    }

    fn set_id(&mut self, id: String) {
        self.id = Some(id);
    }

    fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    fn get_sbo_term(&self) -> Option<&String> {
        self.sbo_term.as_ref()
    }

    fn set_sbo_term(&mut self, sbo_term: String) {
        self.sbo_term = Some(sbo_term);
    }

    fn get_metaid(&self) -> Option<&String> {
        self.metaid.as_ref()
    }

    fn set_metaid(&mut self, metaid: String) {
        self.metaid = Some(metaid);
    }
}
