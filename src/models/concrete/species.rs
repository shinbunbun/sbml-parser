use serde::{Deserialize, Serialize};

use crate::models::{
    primitive::{SIdRef, UnitSidRef},
    r#abstract::s_base::SBase,
};

// Section4.6
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct Species {
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
    #[serde(flatten)]
    pub s_base: SBase, //todo: pub id: SId
}
