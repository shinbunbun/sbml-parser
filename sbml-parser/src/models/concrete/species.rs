use serde::{Deserialize, Serialize};

use crate::models::primitive::{SBOTerm, SId};
use crate::models::primitive::{SIdRef, UnitSidRef};
use crate::models::r#abstract::s_base::SBaseAttributes;

// Section4.6
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct Species {
    #[serde(rename = "@id")]
    pub id: SId,
    #[serde(rename = "@sboTerm", skip_serializing_if = "Option::is_none")]
    pub sbo_term: Option<SBOTerm>,
    #[serde(rename = "@compartment")]
    pub compartment: SIdRef, //todo: The required attribute compartment, of type SIdRef, is used to identify the compartment in which the species is located.
    #[serde(rename = "@initialAmount", skip_serializing_if = "Option::is_none")]
    pub initial_amount: Option<f64>,
    #[serde(
        rename = "@initialConcentration",
        skip_serializing_if = "Option::is_none"
    )]
    pub initial_concentration: Option<f64>, //todo: 4.6.4 The initialAmount, initialConcentration, and substanceUnits attributes
    #[serde(rename = "@substanceUnits", skip_serializing_if = "Option::is_none")]
    pub substance_units: Option<UnitSidRef>,
    #[serde(rename = "@hasOnlySubstanceUnits")]
    pub has_only_substance_units: bool,
    #[serde(rename = "@boundaryCondition")]
    pub boundary_condition: bool,
    #[serde(rename = "@constant")]
    pub constant: bool,
    #[serde(rename = "@conversionFactor", skip_serializing_if = "Option::is_none")]
    pub conversion_factor: Option<SIdRef>,
}

impl SBaseAttributes for Species {
    fn get_id(&self) -> Option<&SId> {
        Some(&self.id)
    }

    fn set_id(&mut self, id: SId) {
        self.id = id;
    }

    fn get_sbo_term(&self) -> Option<&String> {
        self.sbo_term.as_ref()
    }

    fn set_sbo_term(&mut self, sbo_term: String) {
        self.sbo_term = Some(sbo_term);
    }
}
