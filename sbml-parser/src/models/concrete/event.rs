use serde::{Deserialize, Serialize};

use crate::models::{
    primitive::{SBOTerm, ID},
    r#abstract::s_base::SBaseAttributes,
};

use super::{
    delay::Delay, list_of_event_assignments::ListOfEventAssignments, priority::Priority,
    trigger::Trigger,
};

// p79/Section4.12.1
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct Event {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<ID>,
    #[serde(rename = "@sboTerm", skip_serializing_if = "Option::is_none")]
    pub sbo_term: Option<SBOTerm>,
    #[serde(rename = "@useValuesFromTriggerTime")]
    pub use_values_from_trigger_time: bool,
    #[serde(rename = "@trigger", skip_serializing_if = "Option::is_none")]
    pub trigger: Option<Trigger>,
    #[serde(rename = "@priority", skip_serializing_if = "Option::is_none")]
    pub priority: Option<Priority>,
    #[serde(rename = "@delay", skip_serializing_if = "Option::is_none")]
    pub delay: Option<Delay>,
    #[serde(
        rename = "@listOfEventAssignments",
        skip_serializing_if = "Option::is_none"
    )]
    pub list_of_event_assignments: Option<ListOfEventAssignments>,
}

impl SBaseAttributes for Event {
    fn get_id(&self) -> Option<&String> {
        self.id.as_ref()
    }

    fn set_id(&mut self, id: String) {
        self.id = Some(id);
    }

    fn get_sbo_term(&self) -> Option<&String> {
        self.sbo_term.as_ref()
    }

    fn set_sbo_term(&mut self, sbo_term: String) {
        self.sbo_term = Some(sbo_term);
    }
}
