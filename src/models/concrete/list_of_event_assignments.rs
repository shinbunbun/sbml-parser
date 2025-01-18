use serde::{Deserialize, Serialize};

use crate::models::r#abstract::s_base::SBase;

use super::event_assignment::EventAssignment;

// p79/Section4.12
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct ListOfEventAssignments {
    #[serde(rename = "eventAssignment")]
    pub event_assignment: Vec<EventAssignment>,
    #[serde(flatten)]
    pub s_base: SBase,
}
