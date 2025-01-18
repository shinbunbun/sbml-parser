use serde::{Deserialize, Serialize};

use crate::models::{primitive::SIdRef, r#abstract::rule::Rule};

// Section 4.9.4
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct RateRule {
    pub variable: SIdRef,
    #[serde(flatten)]
    pub rule: Rule,
}
