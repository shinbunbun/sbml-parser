use serde::{Deserialize, Serialize};

use crate::models::primitive::{SId, SIdRef, UnitSidRef};

// Section4.6
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct Species {
    pub id: SId,
    pub compartment: SIdRef, //todo: The required attribute compartment, of type SIdRef, is used to identify the compartment in which the species is located.
    #[serde(rename = "initialAmount")]
    pub initial_amount: Option<f64>,
    #[serde(rename = "initialConcentration")]
    pub initial_concentration: Option<f64>, //todo: 4.6.4 The initialAmount, initialConcentration, and substanceUnits attributes
    #[serde(rename = "substanceUnits")]
    pub substance_units: Option<UnitSidRef>,
    #[serde(rename = "hasOnlySubstanceUnits")]
    pub has_only_substance_units: bool,
    #[serde(rename = "boundaryCondition")]
    pub boundary_condition: bool,
    pub constant: bool,
    #[serde(rename = "conversionFactor")]
    pub conversion_factor: Option<SIdRef>,
}
