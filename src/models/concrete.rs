use serde::{Deserialize, Serialize};

use super::primitive::{SBOTerm, SId, SIdRef, UnitSidRef};

type Math = String;

// p46/Section4.5
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct Compoartment {
    pub id: String, // todo: Sid
    #[serde(rename = "spatialDimensions")]
    pub spatial_dimensions: Option<f64>,
    pub size: Option<f64>, // todo: A compartment’s size is set by its size attribute exactly once. If the compartment’s attribute constant has21 the value “true”, then the compartment’s size is fixed and cannot be changed except by an InitialAssignment in22 the model. The approach of using an InitialAssignment differs from setting the size attribute in that size can23 only be used to set the compartment size to a literal floating-point number, whereas InitialAssignment allows24 the value to be set using an arbitrary mathematical expression (which, thanks to MathML’s expressiveness,25 may evaluate to a rational number).
    pub units: String,     // todo: Optional<UnitSidRef>, 4.5.4 The units attribute
    ///  Compartment also has a mandatory boolean attribute called constant that indicates whether the compart-40 ment’s size stays constant or can vary during a simulation. A value of “false” indicates the compartment’ size can be changed by other constructs in SBML. A value of “true” indicates the compartment’s size1 cannot be changed by any construct except InitialAssignment.
    pub constant: bool,
    #[serde(rename = "sboTerm")]
    pub sob_term: Option<SBOTerm>,
}

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

// p53/Section4.7
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct Parameter {
    pub id: SId,
    pub value: Option<f64>,
    pub units: Option<UnitSidRef>,
    pub constant: bool,
    #[serde(rename = "sboTerm")]
    pub sbo_term: Option<SBOTerm>,
}

// Section4.8
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct InitialAssignment {
    pub id: Option<SId>,
    pub symbol: SIdRef,
    pub math: Option<Math>,
    #[serde(rename = "sboTerm")]
    pub sbo_term: Option<SBOTerm>,
}
