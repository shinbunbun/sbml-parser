// section3.1

use serde::{Deserialize, Serialize};

pub type ID = String;
pub type SId = String;
pub type SIdRef = String;
pub type UnitSidRef = UnitSid;
pub type LocalSId = String;
pub type SBOTerm = String;

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
