use serde::{Deserialize, Serialize};

use crate::models::{primitive::UnitSidRef, r#abstract::s_base::SBase};

// p53/Section4.7
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct Parameter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub units: Option<UnitSidRef>,
    pub constant: bool,
    #[serde(flatten)]
    pub s_base: SBase, //todo: pub id: SId
}
