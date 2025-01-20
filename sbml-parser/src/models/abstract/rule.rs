use crate::models::concrete::math::Math;

use super::s_base::SBaseAttributes;

// use super::s_base::SBase;

// Section 4.9
/* #[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct Rule {
    #[serde(flatten)]
    pub s_base: SBase,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub math: Option<Math>,
} */

pub trait RuleAttributes: SBaseAttributes {
    fn get_math(&self) -> Option<&Math> {
        None
    }
    fn set_math(&mut self, _math: Math) {}

    fn get_id(&self) -> Option<&String>;
    fn set_id(&mut self, id: String);

    fn get_name(&self) -> Option<&String>;
    fn set_name(&mut self, name: String);

    fn get_sbo_term(&self) -> Option<&String>;
    fn set_sbo_term(&mut self, sbo_term: String);
}
