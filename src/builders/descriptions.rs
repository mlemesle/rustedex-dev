use std::collections::HashMap;

use anyhow::{Context, Result};
use async_trait::async_trait;
use rustemon::{
    client::RustemonClient,
    model::resource::{FlavorText, NamedApiResource},
};
use serde::Serialize;

use crate::find_by_lang::FindWordingByLang;

use super::Builder;

#[derive(Serialize)]
pub(crate) struct Descriptions {
    generation_id_to_names: HashMap<String, String>,
    generation_id_to_version_name_and_flavor_text: HashMap<String, Vec<VersionNameWithFlavorText>>,
}

#[derive(Serialize)]
pub(crate) struct VersionNameWithFlavorText {
    version_name: String,
    flavor_text: String,
}

impl Descriptions {
    fn new() -> Self {
        Self {
            generation_id_to_names: HashMap::new(),
            generation_id_to_version_name_and_flavor_text: HashMap::new(),
        }
    }
}

#[async_trait]
impl Builder<String> for Descriptions {
    async fn build(id: &String, rc: &RustemonClient, lang: &str) -> Result<Self> {
        let flavor_text_entries = rustemon::pokemon::pokemon::get_by_name(id, rc)
            .await?
            .species
            .with_context(|| format!("No species for {}", id))?
            .follow(rc)
            .await?
            .flavor_text_entries
            .unwrap_or_default();

        let mut descriptions = Descriptions::new();

        for flavor_text_entry in flavor_text_entries {
            if let FlavorText {
                flavor_text: Some(flavor_text),
                language:
                    Some(NamedApiResource {
                        name: Some(language),
                        ..
                    }),
                version:
                    Some(NamedApiResource {
                        name: Some(version_id),
                        ..
                    }),
            } = flavor_text_entry
            {
                if *lang != language {
                    continue;
                }

                let version = rustemon::games::version::get_by_name(&version_id, rc).await?;
                let version_name = version
                    .names
                    .unwrap_or_default()
                    .find_by_lang(lang)
                    .with_context(|| format!("No version name in {} for {}", lang, version_id))?;

                let generation = version
                    .version_group
                    .with_context(|| format!("No version group for {}", version_id))?
                    .follow(rc)
                    .await?
                    .generation
                    .with_context(|| format!("No generation for version {}", version_id))?
                    .follow(rc)
                    .await?;

                let generation_id = generation.name.unwrap();
                let generation_name = generation
                    .names
                    .unwrap_or_default()
                    .find_by_lang(lang)
                    .with_context(|| {
                        format!("No generation name in {} for {}", lang, generation_id)
                    })?;

                descriptions
                    .generation_id_to_names
                    .insert(generation_id.clone(), generation_name);

                let flavor_text = flavor_text
                    .chars()
                    .map(|c| match c {
                        '\u{000c}' | '\n' => ' ',
                        _ => c,
                    })
                    .collect();
                descriptions
                    .generation_id_to_version_name_and_flavor_text
                    .entry(generation_id)
                    .or_insert_with(Vec::new)
                    .push(VersionNameWithFlavorText {
                        version_name,
                        flavor_text,
                    });
            }
        }

        Ok(descriptions)
    }
}
