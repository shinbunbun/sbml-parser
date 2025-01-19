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
    #[serde(rename = "substanceUnits", skip_serializing_if = "Option::is_none")]
    substance_units: Option<UnitSidRef>,
    #[serde(rename = "timeUnits", skip_serializing_if = "Option::is_none")]
    time_units: Option<UnitSidRef>,
    #[serde(rename = "volumeUnits", skip_serializing_if = "Option::is_none")]
    volume_units: Option<UnitSidRef>,
    #[serde(rename = "areaUnits", skip_serializing_if = "Option::is_none")]
    area_units: Option<UnitSidRef>,
    #[serde(rename = "lengthUnits", skip_serializing_if = "Option::is_none")]
    length_units: Option<UnitSidRef>,
    #[serde(rename = "extentUnits", skip_serializing_if = "Option::is_none")]
    extent_units: Option<UnitSidRef>,
    #[serde(rename = "conversionFactor", skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    function_definitions: Option<Vec<FunctionDefinition>>,
}
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct ListOfUnitDefinitions {
    #[serde(skip_serializing_if = "Option::is_none")]
    unit_definitions: Option<Vec<UnitDefinition>>,
}
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct ListOfCompartments {
    #[serde(skip_serializing_if = "Option::is_none")]
    compartments: Option<Vec<Compartment>>,
}
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct ListOfSpecies {
    #[serde(skip_serializing_if = "Option::is_none")]
    species: Option<Vec<Species>>,
}
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct ListOfParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    parameters: Option<Vec<Parameter>>,
}
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct ListOfInitialAssignments {
    #[serde(skip_serializing_if = "Option::is_none")]
    initial_assignments: Option<Vec<InitialAssignment>>,
}
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct ListOfConstraints {
    #[serde(skip_serializing_if = "Option::is_none")]
    constraints: Option<Vec<Constraint>>,
}
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct ListOfReactions {
    #[serde(skip_serializing_if = "Option::is_none")]
    reactions: Option<Vec<Reaction>>,
}
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct ListOfEvents {
    #[serde(skip_serializing_if = "Option::is_none")]
    events: Option<Vec<Event>>,
}
