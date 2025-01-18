use serde::{Deserialize, Serialize};

use crate::models::r#abstract::rule::Rule;

// Section 4.9.2
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct AlgebraicRule {
    #[serde(flatten)]
    pub rule: Rule,
}
