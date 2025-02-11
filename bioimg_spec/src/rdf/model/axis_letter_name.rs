use std::collections::BTreeSet;

#[derive(thiserror::Error, Debug)]
pub enum LegacyAxisIdParsingError{
    #[error("Character cannot be a legacy axis id: {character}")]
    Invalid{character: char},
    #[error("Axis '{0}' apperas multiple times")]
    Repeated(AxisLetterName),
}

#[derive(serde::Serialize, serde::Deserialize, derive_more::Display)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub enum AxisLetterName{
    #[serde(rename="c")] #[display("c")]
    C,
    #[serde(rename="z")] #[display("z")]
    Z,
    #[serde(rename="y")] #[display("y")]
    Y,
    #[serde(rename="x")] #[display("c")]
    X
}

impl TryFrom<char> for AxisLetterName{
    type Error = LegacyAxisIdParsingError;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        Ok(match c{
            'c' => Self::C,
            'z' => Self::Z,
            'y' => Self::Y,
            'x' => Self::X,
            _ => return Err(LegacyAxisIdParsingError::Invalid{character: c})
        })
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
#[serde(try_from="String")]
#[serde(into="String")]
pub struct LegacyAxisIds(BTreeSet<AxisLetterName>);

impl TryFrom<String> for LegacyAxisIds{
    type Error = LegacyAxisIdParsingError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut out = BTreeSet::<AxisLetterName>::new();
        for c in value.chars(){
            let axis = AxisLetterName::try_from(c)?;
            if !out.insert(axis){
                return Err(LegacyAxisIdParsingError::Repeated(axis))
            }
        }
        Ok(Self(out))
    }
}

impl From<LegacyAxisIds> for String{
    fn from(value: LegacyAxisIds) -> Self {
        value.0.iter().map(|ax| ax.to_string()).collect()
    }
}
