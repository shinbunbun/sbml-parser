pub mod models;
pub mod unit;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use models::concrete::{
        model::{ListOfSpecies, Model},
        species::Species,
    };

    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn sbml_test() {
        /* let sbml_str = r#"
                    <?xml version="1.0" encoding="UTF-8"?>
        <sbml xmlns="http://www.sbml.org/sbml/level3/version1/core" level="3" version="1">
            <model>
                <listOfUnitDefinitions>
                    <unitDefinition id="per_second">
                        <listOfUnits>
                            <unit kind="second" exponent="-1" scale="0" multiplier="1"/>
                        </listOfUnits>
                    </unitDefinition>
                </listOfUnitDefinitions>
                <listOfCompartments>
                    <compartment id="cell" size="1" spatialDimensions="3" constant="true"
                                 units="litre"/>
                </listOfCompartments>
                <listOfSpecies>
                    <species id="X0" compartment="cell" initialConcentration="1"
                             hasOnlySubstanceUnits="false" boundaryCondition="false"
                             constant="false" substanceUnits="mole"/>
                    <species id="X1" compartment="cell" initialConcentration="0"
                             hasOnlySubstanceUnits="false" boundaryCondition="false"
                             constant="false" substanceUnits="mole"/>
                    <species id="T"  compartment="cell" initialConcentration="0"
                             hasOnlySubstanceUnits="false" boundaryCondition="false"
                             constant="false" substanceUnits="mole"/>
                    <species id="S1" compartment="cell" initialConcentration="0"
                             hasOnlySubstanceUnits="false" boundaryCondition="false"
                             constant="false" substanceUnits="mole"/>
                    <species id="S2" compartment="cell" initialConcentration="0"
                             hasOnlySubstanceUnits="false" boundaryCondition="false"
                             constant="false" substanceUnits="mole"/>
                </listOfSpecies>
                <listOfParameters>
                    <parameter id="Keq" value="2.5" units="dimensionless" constant="true"/>
                </listOfParameters>
                <listOfRules>
                    <assignmentRule variable="S2">
                        <math xmlns="http://www.w3.org/1998/Math/MathML">
                            <apply>
                                <times/>
                                <ci> Keq </ci>
                                <ci> S1 </ci>
                            </apply>
                        </math>
                    </assignmentRule>
                    <algebraicRule>
                        <math xmlns="http://www.w3.org/1998/Math/MathML">
                            <apply>
                                <minus/>
                                <apply>
                                    <plus/>
                                    <ci> S2 </ci>
                                    <ci> S1 </ci>
                                </apply>
                                <ci> T </ci>
                            </apply>
                        </math>
                    </algebraicRule>
                </listOfRules>
                <listOfReactions>
                    <reaction id="in" reversible="false" fast="false">
                        <listOfReactants>
                            <speciesReference species="X0" stoichiometry="1" constant="true"/>
                        </listOfReactants>
                        <listOfProducts>
                            <speciesReference species="T" stoichiometry="1" constant="true"/>
                        </listOfProducts>
                        <kineticLaw>
                            <math xmlns="http://www.w3.org/1998/Math/MathML">
                                <apply>
                                    <times/>
                                    <ci> k1 </ci>
                                    <ci> X0 </ci>
                                    <ci> cell </ci>
                                </apply>
                            </math>
                            <listOfLocalParameters>
                                <localParameter id="k1" value="0.1" units="per_second"/>
                            </listOfLocalParameters>
                        </kineticLaw>
                    </reaction>
                    <reaction id="out" reversible="false" fast="false">
                        <listOfReactants>
                            <speciesReference species="T" stoichiometry="1" constant="true"/>
                        </listOfReactants>
                        <listOfProducts>
                            <speciesReference species="X1" stoichiometry="1" constant="true"/>
                        </listOfProducts>
                        <listOfModifiers>
                                <modifierSpeciesReference species="S2"/>
                        </listOfModifiers>
                        <kineticLaw>
                            <math xmlns="http://www.w3.org/1998/Math/MathML">
                                <apply>
                                    <times/>
                                    <ci> k2 </ci>
                                    <ci> S2 </ci>
                                    <ci> cell </ci>
                                </apply>
                            </math>
                            <listOfLocalParameters>
                                <localParameter id="k2" value="0.15" units="per_second"/>
                            </listOfLocalParameters>
                        </kineticLaw>
                    </reaction>
                </listOfReactions>
            </model>
        </sbml>"#; */
        let sbml_str = r#"
            <species id="X0" compartment="cell" initialConcentration="1"
                     hasOnlySubstanceUnits="false" boundaryCondition="false"
                     constant="false" substanceUnits="mole"/>"#;
        let parsed_sbml: Species = quick_xml::de::from_str(sbml_str).unwrap();
        println!("{:?}", parsed_sbml);
    }
}
