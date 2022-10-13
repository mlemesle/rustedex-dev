use std::iter::Iterator;

use rustemon::model::pokemon::Genus;

pub(crate) const EN: &str = "en";

pub(crate) trait FindWordingByLang
where
    Self: IntoIterator,
{
    fn find_by_lang(&self, lang: &str) -> Option<String>;
}

impl FindWordingByLang for Vec<Genus> {
    fn find_by_lang(&self, lang: &str) -> Option<String> {
        self.iter()
            .find(|genus| genus.language.as_ref().unwrap().name == Some(lang.into()))
            .map(|genus| genus.genus.clone())
            .flatten()
    }
}
