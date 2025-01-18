use serde::{Deserialize, Serialize};

use crate::models::r#abstract::s_base::SBase;

use super::{math::Math, message::Message};

// Section 4.10
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct Constraint {
    pub math: Option<Math>,
    pub message: Option<Message>,
    #[serde(flatten)]
    pub s_base: SBase,
}
