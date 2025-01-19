use serde::{Deserialize, Serialize};

use crate::models::concrete::math::Math;

use super::s_base::SBase;

// Section 4.9
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct Rule {
    #[serde(flatten)]
    pub s_base: SBase,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub math: Option<Math>,
}
