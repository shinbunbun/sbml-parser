use serde::{Deserialize, Serialize};

use crate::unit::UnitSidRef;

type Sid = String;
type SidRef = String; //todo: This type is derived from SId, but with the restriction that the value of an attribute having type SIdRef must equal the value of some SId attribute in the model where it appears. In other words, a SIdRef value must be an existing identifier in a model.

// Section4.6
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct Species {
    pub id: Sid,
    pub compartment: SidRef, //todo: The required attribute compartment, of type SIdRef, is used to identify the compartment in which the species is located.
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
    pub conversion_factor: Option<SidRef>,
}
