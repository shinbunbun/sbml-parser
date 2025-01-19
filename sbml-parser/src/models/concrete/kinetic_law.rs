use serde::{Deserialize, Serialize};

use crate::models::r#abstract::s_base::SBase;

use super::{local_parameter::LocalParameter, math::Math};

// Section 4.11.5
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct KineticLaw {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub math: Option<Math>,
    #[serde(rename = "listOfLocalParameters")]
    pub list_of_local_parameters: Option<ListOfLocalParameters>,
    #[serde(flatten)]
    pub s_base: SBase,
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct ListOfLocalParameters {
    #[serde(rename = "localParameter", skip_serializing_if = "Option::is_none")]
    pub local_parameter: Option<Vec<LocalParameter>>,
    #[serde(flatten)]
    pub s_base: SBase,
}
