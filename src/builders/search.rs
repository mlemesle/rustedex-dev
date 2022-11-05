use std::path::PathBuf;

use anyhow::{Context, Result};
use async_trait::async_trait;
use rustemon::{client::RustemonClient, Follow};
use serde::{Deserialize, Serialize};
use unidecode::unidecode;

use super::Builder;
use crate::find_by_lang::FindWordingByLang;

#[derive(Serialize, Deserialize)]
pub(crate) struct SearchElement {
    id: i64,
    search_name_en: String,
    search_name_fr: String,
    display_name: String,
    sprite: String,
    path: PathBuf,
}

#[derive(Serialize)]
pub(crate) struct Search {
    search_elements: Vec<SearchElement>,
}

#[async_trait]
impl Builder<Vec<(String, PathBuf)>> for Search {
    async fn build(data: &Vec<(String, PathBuf)>, rc: &RustemonClient, lang: &str) -> Result<Self> {
        let mut search_elements = Vec::with_capacity(data.len());

        for (pokemon_id, path) in data {
            let pokemon = rustemon::pokemon::pokemon::get_by_name(pokemon_id, rc).await?;

            let pokemon_species = pokemon
                .species
                .with_context(|| format!("No species for {}", pokemon_id))?
                .follow(rc)
                .await?;

            let pokemon_index = pokemon_species
                .pokedex_numbers
                .unwrap_or_default()
                .iter()
                .find(|pokemon_number| {
                    pokemon_number
                        .pokedex
                        .as_ref()
                        .unwrap()
                        .name
                        .as_ref()
                        .unwrap()
                        == "national"
                })
                .map(|pokemon_number| pokemon_number.entry_number.unwrap())
                .unwrap_or(9999);

            let names = pokemon_species.names.unwrap_or_default();
            let display_name = names
                .find_by_lang(lang)
                .with_context(|| format!("No {} name for {}", lang, pokemon_id))?;
            let search_name_fr = names
                .find_by_lang("fr")
                .with_context(|| format!("No {} name for {}", "fr", pokemon_id))?
                .to_lowercase();
            let search_name_fr = unidecode(&search_name_fr);

            let pokemon_sprite = pokemon
                .sprites
                .with_context(|| format!("No sprites for {}", pokemon_id))?
                .front_default
                .unwrap_or_else(|| {
                    format!(
                        "https://raw.githubusercontent.com/PokeAPI/sprites/master/sprites/pokemon/{}.png",
                        pokemon.id.unwrap()
                    )
                });

            search_elements.push(SearchElement {
                id: pokemon_index,
                search_name_en: display_name.to_lowercase(),
                search_name_fr,
                display_name,
                sprite: pokemon_sprite,
                path: path.clone(),
            });
        }

        Ok(Self { search_elements })
    }
}
