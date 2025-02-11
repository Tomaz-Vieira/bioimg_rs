use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::rdf::BoundedString;

use super::{orcid::Orcid, slashless_string::SlashlessString};

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Author {
    pub name: SlashlessString<String>,        // (Name→String) Full name. FIXME: disallow / and \.
    pub affiliation: Option<String>, // (String) Affiliation.
    pub email: Option<String>,       // FIXME: make a parser here (Email) E-Mail
    pub github_user: Option<String>, // (String) GitHub user name.
    pub orcid: Option<Orcid>,
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct Author2 {
    pub name: BoundedString<1, 1024>,                // (Name→String) Full name.
    pub affiliation: Option<BoundedString<1, 1024>>, // (String) Affiliation.
    pub email: Option<BoundedString<1, 1024>>,       // FIXME: make a parser here (Email) E-Mail
    pub github_user: Option<BoundedString<1, 1024>>, // (String) GitHub user name.
    pub orcid: Option<Orcid>,
}

impl Display for Author2{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)?;
        if let Some(email) = &self.email{
            write!(f, " 📧{email}")?;
        }
        if let Some(github_user) = &self.github_user{
            write!(f, " github: {github_user}")?;
        }
        Ok(())
    }
}
