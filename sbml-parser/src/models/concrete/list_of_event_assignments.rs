use serde::{Deserialize, Serialize};

use crate::models::r#abstract::s_base::SBaseAttributes;

use super::event_assignment::EventAssignment;

// p79/Section4.12
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct ListOfEventAssignments {
    #[serde(rename = "@eventAssignment", skip_serializing_if = "Option::is_none")]
    pub event_assignment: Option<Vec<EventAssignment>>,
}

impl SBaseAttributes for ListOfEventAssignments {}
