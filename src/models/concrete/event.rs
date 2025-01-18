use serde::{Deserialize, Serialize};

use crate::models::r#abstract::s_base::SBase;
use super::{delay::Delay, list_of_event_assignments::ListOfEventAssignments, priority::Priority, trigger::Trigger};

// p79/Section4.12
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct Event {
    #[serde(rename = "useValuesFromTriggerTime")]
    pub use_values_from_trigger_time: bool,
    pub trigger: Option<Trigger>,
    pub priority: Option<Priority>,
    pub delay: Option<Delay>,
    #[serde(rename = "ListOfEventAssignments")]
    pub list_of_event_assignments: Option<ListOfEventAssignments>,
    #[serde(flatten)]
    pub s_base: SBase, // todo pub id: Option<SId>
} //todo
