use serde::{Deserialize, Serialize};

use crate::models::r#abstract::s_base::SBase;

// p46/Section4.5
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct Compartment {
    #[serde(rename = "spatialDimensions", skip_serializing_if = "Option::is_none")]
    pub spatial_dimensions: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<f64>,
    pub units: String,
    ///  Compartment also has a mandatory boolean attribute called constant that indicates whether the compart-40 ment’s size stays constant or can vary during a simulation. A value of “false” indicates the compartment’ size can be changed by other constructs in SBML. A value of “true” indicates the compartment’s size1 cannot be changed by any construct except InitialAssignment.
    pub constant: bool,
    #[serde(flatten)]
    pub s_base: SBase, //todo: pub id: SId
}
