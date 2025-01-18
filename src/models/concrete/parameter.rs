use serde::{Deserialize, Serialize};

use crate::models::{primitive::UnitSidRef, r#abstract::s_base::SBase};

// p53/Section4.7
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct Parameter {
    pub value: Option<f64>,
    pub units: Option<UnitSidRef>,
    pub constant: bool,
    #[serde(flatten)]
    pub s_base: SBase, //todo: pub id: SId
}
