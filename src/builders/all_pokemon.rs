use std::path::PathBuf;

use anyhow::Result;
use async_trait::async_trait;
use rustemon::client::RustemonClient;
use serde::Serialize;

use super::Builder;

#[derive(Serialize)]
pub(crate) struct PokemonNameAndPath {
    name: String,
    path: PathBuf,
}

#[derive(Serialize)]
pub(crate) struct AllPokemon {
    pokemon_names_and_paths: Vec<PokemonNameAndPath>,
}

#[async_trait]
impl Builder<Vec<(String, PathBuf)>> for AllPokemon {
    async fn build(data: Vec<(String, PathBuf)>, _rc: &RustemonClient) -> Result<Self> {
        let pokemon_names_and_paths = data
            .into_iter()
            .map(|(name, path)| PokemonNameAndPath { name, path })
            .collect();

        Ok(Self {
            pokemon_names_and_paths,
        })
    }
}
