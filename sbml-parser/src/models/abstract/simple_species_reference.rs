use super::s_base::SBaseAttributes;

pub trait SimpleSpeciesReferenceAttributes: SBaseAttributes {
    fn get_species(&self) -> &String;
    fn set_species(&mut self, species: String);

    fn get_sbo_term(&self) -> Option<&String> {
        None
    }
    fn set_sbo_term(&mut self, _sbo_term: String) {}
}
