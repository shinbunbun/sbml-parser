use serde::{Deserialize, Serialize};

// p79/Section4.12
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct ListOfEventAssignments {
    #[serde(rename = "EventAssignment")]
    pub event_assignment: Vec<EventAssignment>,
    #[serde(flatten)]
    pub s_base: SBasee,
}
