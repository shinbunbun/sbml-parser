use serde::{Deserialize, Serialize};

// Section 4.11.3
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct SpeciesReference {
    pub stoichiometry: Option<f64>,
    pub constant: bool,
}
