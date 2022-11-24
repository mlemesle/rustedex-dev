use std::collections::HashMap;

use anyhow::Result;
use async_trait::async_trait;
use rustemon::{client::RustemonClient, Follow};
use serde::Serialize;

use crate::{find_by_lang::FindWordingByLang, utils};

use super::Builder;

#[derive(Serialize)]
pub(crate) struct Locations {
    version_group_id_and_names: Vec<(&'static str, &'static str)>,
    locations_by_version: HashMap<String, Vec<String>>,
}

/// TODO: Remove once data is present
impl Default for Locations {
    fn default() -> Self {
        Self {
            version_group_id_and_names: vec![],
            locations_by_version: HashMap::new(),
        }
    }
}

#[async_trait]
impl Builder<String> for Locations {
    async fn build(id: &String, rc: &RustemonClient, lang: &str) -> Result<Self> {
        let pokemon_id = rustemon::pokemon::pokemon::get_by_name(id, rc).await?.id;

        let location_area_encounters =
            rustemon::pokemon::pokemon::encounters::get_by_id(pokemon_id, rc).await?;

        let mut locations_by_version: HashMap<String, Vec<String>> = HashMap::new();

        for location_area_encounter in location_area_encounters {
            let location_area_name = location_area_encounter
                .location_area
                .follow(rc)
                .await?
                .names
                .find_by_lang(lang)
                .unwrap_or_else(|| "Location area name not found".to_string());

            for version_detail in location_area_encounter.version_details {
                let version_id = version_detail.version.name;
                locations_by_version
                    .entry(version_id)
                    .or_default()
                    .push(location_area_name.clone());
            }
        }

        let version_group_id_and_names =
            utils::get_version_group_id_and_names(locations_by_version.keys().collect());

        Ok(Locations {
            version_group_id_and_names,
            locations_by_version,
        })
    }
}
