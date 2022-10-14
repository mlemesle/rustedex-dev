use std::iter::Iterator;

use rustemon::model::{pokemon::Genus, resource::Name};

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

impl FindWordingByLang for Vec<Name> {
    fn find_by_lang(&self, lang: &str) -> Option<String> {
        self.iter()
            .find(|name| name.language.as_ref().unwrap().name == Some(lang.into()))
            .map(|name| name.name.clone())
            .flatten()
    }
}
