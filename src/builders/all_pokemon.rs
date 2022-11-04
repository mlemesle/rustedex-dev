use std::path::PathBuf;

use anyhow::{Context, Result};
use async_trait::async_trait;
use rustemon::{client::RustemonClient, Follow};
use serde::Serialize;

use super::Builder;
use crate::find_by_lang::FindWordingByLang;

#[derive(Serialize)]
pub(crate) struct PokemonElement {
    id: i64,
    display_name: String,
    sprite: String,
    path: PathBuf,
}

#[derive(Serialize)]
pub(crate) struct AllPokemon {
    pokemon_elements: Vec<PokemonElement>,
}

#[async_trait]
impl Builder<Vec<(String, PathBuf)>> for AllPokemon {
    async fn build(data: &Vec<(String, PathBuf)>, rc: &RustemonClient, lang: &str) -> Result<Self> {
        let mut pokemon_elements = Vec::with_capacity(data.len());

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

            let pokemon_sprite = pokemon
                .sprites
                .with_context(|| format!("No sprites for {}", pokemon_id))?
                .front_default
                .unwrap_or_else(|| {
                    format!(
                        "/rustedex/assets/images/sprites/{}.png",
                        pokemon.id.unwrap()
                    )
                });

            pokemon_elements.push(PokemonElement {
                id: pokemon_index,
                display_name,
                sprite: pokemon_sprite,
                path: path.clone(),
            });
        }

        Ok(Self { pokemon_elements })
    }
}
