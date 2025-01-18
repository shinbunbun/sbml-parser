use serde::{Deserialize, Serialize};

use crate::models::{primitive::UnitSid, r#abstract::s_base::SBase};

/// A single Unit object instance takes one of the base units from Table 2 and specifies how it should be transformed.
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct Unit {
    pub kind: UnitSid,
    pub exponent: f64,
    pub scale: i64,
    pub multiplier: f64,
    #[serde(flatten)]
    pub s_base: SBase,
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct ListOfUnits {
    #[serde(flatten)]
    pub units: Vec<Unit>,
    #[serde(flatten)]
    pub s_base: SBase,
}

/// A UnitDefinition object instance combines one or more Unit objects to define a new, composed unit, u. The new unit u created by a UnitDefinition is defined as the product of all the Unit objects contained in the ListOfUnits 2 within the UnitDefinition object instance
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct UnitDefinition {
    #[serde(rename = "listOfUnits")]
    pub list_of_units: ListOfUnits,
    #[serde(flatten)]
    pub s_base: SBase, //todo: pub id: UnitSid
}
