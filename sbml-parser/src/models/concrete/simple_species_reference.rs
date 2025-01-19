use serde::{Deserialize, Serialize};

use crate::models::{primitive::SIdRef, r#abstract::s_base::SBase};

// Section 4.11.2
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct SimpleSpeciesReference {
    pub species: SIdRef,
    #[serde(flatten)]
    pub s_base: SBase,
}
