use bioimg_spec::rdf::{author::Author2, bounded_string::{BoundedString, BoundedStringParsingError}, orcid::{Orcid, OrcidParsingError}};

use super::{StagingString, StagingOptString};

pub type ConfString = BoundedString<1, 1023>;

#[derive(thiserror::Error, Debug)]
pub enum Author2ParsingError{
    #[error("{0}")]
    FieldError(#[from] #[source] BoundedStringParsingError),
    #[error("{0}")]
    BadOrcid(#[from] #[source] OrcidParsingError),
}

#[derive(Default)]
pub struct StagingAuthor2{
    staging_name: StagingString< ConfString >,                // (Name→String) Full name.
    staging_affiliation: StagingOptString< ConfString >, // (String) Affiliation.
    staging_email: StagingOptString<ConfString>,       // FIXME: make a parser here (Email) E-Mail
    staging_github_user: StagingOptString<ConfString>, // (String) GitHub user name.
    staging_orcid: StagingOptString<Orcid>,
}

impl StagingAuthor2{
    pub fn draw_and_update(&mut self, ui: &mut egui::Ui) -> Result<Author2, Author2ParsingError>{
        let name = ui.horizontal(|ui|{
            ui.label("Name");
            self.staging_name.draw_and_update(ui)
        }).inner;
        let affiliation = ui.horizontal(|ui|{
            ui.label("Affiliation");
            self.staging_affiliation.draw_and_update(ui)
        }).inner;
        let email = ui.horizontal(|ui|{
            ui.label("Email");
            self.staging_email.draw_and_update(ui)
        }).inner;
        let github_user = ui.horizontal(|ui|{
            ui.label("Github User");
            self.staging_github_user.draw_and_update(ui)
        }).inner;
        let orcid = ui.horizontal(|ui|{
            ui.label("Orcid");
            self.staging_orcid.draw_and_update(ui)
        }).inner;

        Ok(Author2{
            name: name?,
            affiliation: affiliation?,
            email: email?,
            github_user: github_user?,
            orcid: orcid?,
        })
    }
}
