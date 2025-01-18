use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub enum UnitSid {
    Ampere,
    Avogadro,
    Becquerel,
    Candela,
    Coulomb,
    Dimensionless,
    Farad,
    Gram,
    Gray,
    Henry,
    Hertz,
    Item,
    Joule,
    Katal,
    Kelvin,
    Kilogram,
    Litre,
    Lumen,
    Lux,
    Metre,
    Mole,
    Newton,
    Ohm,
    Pascal,
    Radian,
    Second,
    Siemens,
    Sievert,
    Streradian,
    Tesla,
    Volt,
    Watt,
    Weber,
}

/// A single Unit object instance takes one of the base units from Table 2 and specifies how it should be transformed.
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct Unit {
    pub kind: UnitSid,
    pub exponent: f64,
    pub scale: i64,
    pub multiplier: f64,
}

pub type ListOfUnits = Vec<Unit>;

/// A UnitDefinition object instance combines one or more Unit objects to define a new, composed unit, u. The new unit u created by a UnitDefinition is defined as the product of all the Unit objects contained in the ListOfUnits 2 within the UnitDefinition object instance
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct UnitDefinition {
    pub id: UnitSid,
    #[serde(rename = "listOfUnits")]
    pub list_of_units: ListOfUnits,
}
