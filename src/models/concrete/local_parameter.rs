use serde::{Deserialize, Serialize};

use crate::models::{primitive::UnitSidRef, r#abstract::s_base::SBase};

// Section 4.11.6
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct LocalParameter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub units: Option<UnitSidRef>,
    #[serde(flatten)]
    pub s_base: SBase, // todo: pub id: SId
}
