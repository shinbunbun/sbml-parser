use serde::{Deserialize, Serialize};

use super::math::Math;
use crate::models::r#abstract::s_base::SBase;

// p79/Section4.12
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct Delay {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub math: Option<Math>,
    #[serde(flatten)]
    pub s_base: SBase, // todo: pub id: Option<SId>
}
