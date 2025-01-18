use serde::{Deserialize, Serialize};

use crate::models::r#abstract::rule::Rule;

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct AlgebraicRule {
    #[serde(flatten)]
    pub rule: Rule,
}
