use serde::{Deserialize, Serialize};

use crate::models::{primitive::SId, r#abstract::s_base::SBase};

use super::math::Math;

// Section 4.3
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct FunctionDefinition {
    pub sid: SId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub math: Option<Math>,
    #[serde(flatten)]
    pub s_base: SBase, //todo: idをrequiredにする(Deserializeをimplする)
}
