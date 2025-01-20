use serde::{Deserialize, Serialize};

use crate::models::{
    primitive::{SBOTerm, SIdRef, ID},
    r#abstract::{rule::RuleAttributes, s_base::SBaseAttributes},
};

use super::math::Math;

// Section 4.9.3
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct AssignmentRule {
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ID>,
    #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "@sboTerm", skip_serializing_if = "Option::is_none")]
    pub sbo_term: Option<SBOTerm>,
    #[serde(rename = "@math")]
    pub math: Option<Math>,
    #[serde(rename = "@variable")]
    pub variable: SIdRef,
}

impl SBaseAttributes for AssignmentRule {}

impl RuleAttributes for AssignmentRule {
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

    fn get_math(&self) -> Option<&Math> {
        self.math.as_ref()
    }

    fn set_math(&mut self, math: Math) {
        self.math = Some(math);
    }
}
