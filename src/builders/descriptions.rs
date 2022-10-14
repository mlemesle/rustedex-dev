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
pub(crate) struct Description {
    version_id: String,
    version_name: String,
}

pub(crate) type Descriptions = Vec<Description>;

#[async_trait]
impl Builder<String> for Descriptions {
    async fn build(id: String, rc: &RustemonClient, lang: &String) -> Result<Self> {
        let flavor_text_entries = rustemon::pokemon::pokemon::get_by_name(&id, rc)
            .await?
            .species
            .with_context(|| format!("No species for {}", id))?
            .follow(rc)
            .await?
            .flavor_text_entries
            .unwrap_or_default();

        let mut result = vec![];

        for flavor_text_entry in flavor_text_entries {
            if let FlavorText {
                flavor_text: Some(text),
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

                let version_name = rustemon::games::version::get_by_name(&version_id, rc)
                    .await?
                    .names
                    .unwrap_or_default()
                    .find_by_lang(lang)
                    .with_context(|| format!("No version name in {} for {}", lang, version_id))?;

                result.push(Description {
                    version_id,
                    version_name,
                });
            }
        }

        Ok(result)
    }
}
