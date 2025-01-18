use serde::{Deserialize, Serialize};

use crate::models::{primitive::SIdRef, r#abstract::rule::Rule};

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct AssignmentRule {
    pub variable: SIdRef,
    #[serde(flatten)]
    pub rule: Rule,
}
