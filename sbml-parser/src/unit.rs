use serde::{Deserialize, Serialize};

use crate::models::{primitive::UnitSid, r#abstract::s_base::SBaseAttributes};

// Section 4.4
/// A single Unit object instance takes one of the base units from Table 2 and specifies how it should be transformed.
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct Unit {
    pub kind: UnitSid,
    pub exponent: f64,
    pub scale: i64,
    pub multiplier: f64,
}

impl SBaseAttributes for Unit {}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct ListOfUnits {
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub units: Option<Vec<Unit>>,
}

impl SBaseAttributes for ListOfUnits {}

// Section 4.4.1
/// A UnitDefinition object instance combines one or more Unit objects to define a new, composed unit, u. The new unit u created by a UnitDefinition is defined as the product of all the Unit objects contained in the ListOfUnits 2 within the UnitDefinition object instance
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct UnitDefinition {
    pub id: UnitSid,
    #[serde(rename = "listOfUnits")]
    pub list_of_units: ListOfUnits,
}

impl SBaseAttributes for UnitDefinition {
    fn get_id(&self) -> Option<&String> {
        todo!()
    }

    fn set_id(&mut self, id: String) {
        self.id = serde_lexpr::from_str(&id).unwrap();
    }
}
