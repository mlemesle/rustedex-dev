use std::{collections::HashMap, path::PathBuf};

use anyhow::Result;
use async_trait::async_trait;
use rustemon::client::RustemonClient;
use serde::Serialize;

use super::Builder;

#[derive(Serialize)]
pub(crate) struct PokemonNameAndPath {
    id: i64,
    name: String,
    path: PathBuf,
}

#[derive(Serialize)]
pub(crate) struct AllPokemon {
    pokemon_names_and_paths_by_letter: HashMap<char, Vec<PokemonNameAndPath>>,
}

#[async_trait]
impl Builder<Vec<(i64, String, PathBuf)>> for AllPokemon {
    async fn build(
        data: Vec<(i64, String, PathBuf)>,
        _rc: &RustemonClient,
        _lang: &str,
    ) -> Result<Self> {
        let pokemon_names_and_paths: Vec<_> = data
            .into_iter()
            .map(|(id, name, path)| PokemonNameAndPath { id, name, path })
            .collect();

        let mut pokemon_names_and_paths_by_letter = HashMap::new();

        for pokemon_name_and_path in pokemon_names_and_paths {
            pokemon_names_and_paths_by_letter
                .entry(
                    pokemon_name_and_path
                        .name
                        .chars()
                        .next()
                        .unwrap_or_default(),
                )
                .or_insert_with(Vec::new)
                .push(pokemon_name_and_path);
        }

        Ok(Self {
            pokemon_names_and_paths_by_letter,
        })
    }
}
