use serde::{Deserialize, Serialize};

use crate::models::{r#abstract::s_base::SBase, primitive::SIdRef};
use super::math::Math;

// p79/Section4.12
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct EventAssignment {
    pub variable: SIdRef,
    pub math: Option<Math>,
    #[serde(flatten)]
    pub s_base: SBase,
}
