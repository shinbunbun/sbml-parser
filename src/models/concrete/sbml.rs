use serde::{Deserialize, Serialize};

use crate::models::r#abstract::s_base::SBase;

use super::model::Model;

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct SBML {
    /// fixed: 3
    pub level: u8,
    /// fixed: 2
    pub version: u8,
    /// fixed: http://www.sbml.org/sbml/level3/version2/core
    pub xmlns: String,
    #[serde(flatten)]
    pub s_base: SBase,
    pub model: Option<Model>,
}
