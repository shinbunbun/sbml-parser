use serde::{Deserialize, Serialize};

use crate::{models::primitive::UnitSidRef, unit::UnitDefinition};

use super::{
    compartment::Compartment, constraint::Constraint, event::Event,
    function_definition::FunctionDefinition, initial_assignment::InitialAssignment,
    parameter::Parameter, reaction::Reaction, species::Species,
};

// Section 4.2
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct Model {
    #[serde(rename = "substanceUnits")]
    substance_units: Option<UnitSidRef>,
    #[serde(rename = "timeUnits")]
    time_units: Option<UnitSidRef>,
    #[serde(rename = "volumeUnits")]
    volume_units: Option<UnitSidRef>,
    #[serde(rename = "areaUnits")]
    area_units: Option<UnitSidRef>,
    #[serde(rename = "lengthUnits")]
    length_units: Option<UnitSidRef>,
    #[serde(rename = "extentUnits")]
    extent_units: Option<UnitSidRef>,
    #[serde(rename = "conversionFactor")]
    conversion_factor: Option<f64>,

    #[serde(rename = "listOfFunctionDefinitions")]
    list_of_function_definitions: ListOfFunctionDefinitions,
    #[serde(rename = "listOfUnitDefinitions")]
    list_of_unit_definitions: ListOfUnitDefinitions,
    #[serde(rename = "listOfCompartments")]
    list_of_compartments: ListOfCompartments,
    #[serde(rename = "listOfSpecies")]
    list_of_species: ListOfSpecies,
    #[serde(rename = "listOfParameters")]
    list_of_parameters: ListOfParameters,
    #[serde(rename = "listOfInitialAssignments")]
    list_of_initial_assignments: ListOfInitialAssignments,
    #[serde(rename = "listOfConstraints")]
    list_of_constraints: ListOfConstraints,
    #[serde(rename = "listOfReactions")]
    list_of_reactions: ListOfReactions,
    #[serde(rename = "listOfEvents")]
    list_of_events: ListOfEvents,
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct ListOfFunctionDefinitions {
    function_definitions: Vec<FunctionDefinition>,
}
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct ListOfUnitDefinitions {
    unit_definitions: Vec<UnitDefinition>,
}
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct ListOfCompartments {
    compartments: Vec<Compartment>,
}
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct ListOfSpecies {
    species: Vec<Species>,
}
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct ListOfParameters {
    parameters: Vec<Parameter>,
}
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct ListOfInitialAssignments {
    initial_assignments: Vec<InitialAssignment>,
}
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct ListOfConstraints {
    constraints: Vec<Constraint>,
}
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct ListOfReactions {
    reactions: Vec<Reaction>,
}
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct ListOfEvents {
    events: Vec<Event>,
}
