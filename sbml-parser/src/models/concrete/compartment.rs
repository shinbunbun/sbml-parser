use serde::{Deserialize, Serialize};

use crate::models::{primitive::SId, r#abstract::s_base::SBaseAttributes};

// p46/Section4.5
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct Compartment {
    #[serde(rename = "@id")]
    pub id: SId,
    #[serde(rename = "@spatialDimensions", skip_serializing_if = "Option::is_none")]
    pub spatial_dimensions: Option<f64>,
    #[serde(rename = "@size", skip_serializing_if = "Option::is_none")]
    pub size: Option<f64>,
    #[serde(rename = "@units")]
    pub units: String,
    ///  Compartment also has a mandatory boolean attribute called constant that indicates whether the compart-40 ment’s size stays constant or can vary during a simulation. A value of “false” indicates the compartment’ size can be changed by other constructs in SBML. A value of “true” indicates the compartment’s size1 cannot be changed by any construct except InitialAssignment.
    #[serde(rename = "@constant")]
    pub constant: bool,
}

impl SBaseAttributes for Compartment {
    fn get_id(&self) -> Option<&SId> {
        Some(&self.id)
    }

    fn set_id(&mut self, id: SId) {
        self.id = id;
    }
}
