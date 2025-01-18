use serde::{Deserialize, Serialize};

use crate::models::concrete::math::Math;

use super::s_base::SBase;

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct Rule {
    #[serde(flatten)]
    pub s_base: SBase,
    pub math: Option<Math>,
}
