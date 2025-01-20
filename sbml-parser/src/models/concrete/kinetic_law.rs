use serde::{Deserialize, Serialize};

use crate::models::{
    primitive::{LocalSId, SBOTerm, ID},
    r#abstract::s_base::SBaseAttributes,
};

use super::{local_parameter::LocalParameter, math::Math};

// Section 4.11.5
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct KineticLaw {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<ID>,
    #[serde(rename = "sboTerm", skip_serializing_if = "Option::is_none")]
    pub sbo_term: Option<SBOTerm>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub math: Option<Math>,
    #[serde(rename = "listOfLocalParameters")]
    pub list_of_local_parameters: Option<ListOfLocalParameters>,
}

impl SBaseAttributes for KineticLaw {
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

// Section 4.11.6
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct ListOfLocalParameters {
    pub id: LocalSId,
    #[serde(rename = "sboTerm", skip_serializing_if = "Option::is_none")]
    pub sbo_term: Option<SBOTerm>,
    #[serde(rename = "localParameter", skip_serializing_if = "Option::is_none")]
    pub local_parameter: Option<Vec<LocalParameter>>,
}

impl SBaseAttributes for ListOfLocalParameters {
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
