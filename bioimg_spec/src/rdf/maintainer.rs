use serde::{Deserialize, Serialize};

use super::{bounded_string::BoundedString, orcid::Orcid, slashless_string::SlashlessString};

pub type MaintainerName = SlashlessString<BoundedString<1, 1023>>;

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct Maintainer {
    pub affiliation: Option<BoundedString<1, 1023>>,
    pub email: Option<BoundedString<1, 1023>>, //FIXME
    pub orcid: Option<Orcid>,
    pub name: Option<MaintainerName>,
    pub github_user: BoundedString<1, 1023>, //FIXME validate this somehow
}

