use serde::{Deserialize, Serialize};

use crate::models::{primitive::SIdRef, r#abstract::s_base::SBase};

use super::math::Math;

// Section4.8
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct InitialAssignment {
    pub symbol: SIdRef,
    pub math: Option<Math>,
    #[serde(flatten)]
    pub s_base: SBase, //todo: pub id: Option<SId>
}
