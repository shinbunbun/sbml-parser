use serde::{Deserialize, Serialize};

use super::{
    delay::Delay, list_of_event_assignments::ListOfEventAssignments, priority::Priority,
    trigger::Trigger,
};
use crate::models::r#abstract::s_base::SBase;

// p79/Section4.12
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct Event {
    #[serde(rename = "useValuesFromTriggerTime")]
    pub use_values_from_trigger_time: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger: Option<Trigger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<Priority>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delay: Option<Delay>,
    #[serde(
        rename = "listOfEventAssignments",
        skip_serializing_if = "Option::is_none"
    )]
    pub list_of_event_assignments: Option<ListOfEventAssignments>,
    #[serde(flatten)]
    pub s_base: SBase, // todo pub id: Option<SId>
}
