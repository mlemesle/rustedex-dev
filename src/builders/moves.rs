use std::collections::HashMap;

use anyhow::{bail, Context, Result};
use async_trait::async_trait;
use rustemon::{client::RustemonClient, model::pokemon::PokemonMove, Follow};
use serde::Serialize;

use crate::find_by_lang::FindWordingByLang;

use super::Builder;

#[derive(Serialize)]
pub(crate) struct Moves {
    pokemon_name: String,
    moves_by_version_group_by_level: HashMap<String, Vec<Move>>,
}

#[derive(Serialize)]
pub(crate) struct Move {
    name: String,
    lvl_learned_at: u8,
}

pub(crate) struct MoveBuild {
    by_level: HashMap<String, Vec<Move>>,
    by_machine: HashMap<String, Vec<Move>>,
}

#[async_trait]
impl Builder<String> for Moves {
    async fn build(id: &String, rc: &RustemonClient, lang: &str) -> Result<Self> {
        let pokemon = rustemon::pokemon::pokemon::get_by_name(id, rc).await?;
        let pokemon_name = pokemon
            .species
            .with_context(|| format!("No species for {}", id))?
            .follow(rc)
            .await?
            .names
            .unwrap_or_default()
            .find_by_lang(lang)
            .with_context(|| format!("No {} name for {}", lang, id))?;

        let moves = match pokemon.moves {
            Some(pokemon_moves) => {
                // let mut moves_by_level = Vec::with_capacity(pokemon_moves.len());

                for pokemon_move in &pokemon_moves {
                    // match MoveBuild::build(pokemon_move, rc, lang).await? {
                    //     Move::ByLevel(move_by_level) => moves_by_level.push(move_by_level),
                    // }
                }

                ()
            }
            None => (),
        };

        Ok(Self {
            pokemon_name: "Prout".into(),
            moves_by_version_group_by_level: HashMap::new(),
        })
    }
}

#[async_trait]
impl Builder<PokemonMove> for MoveBuild {
    async fn build(pokemon_move: &PokemonMove, rc: &RustemonClient, lang: &str) -> Result<Self> {
        let move_ = pokemon_move
            .move_
            .as_ref()
            .with_context(|| format!("No move for {:?}", pokemon_move))?
            .follow(rc)
            .await?;

        let move_name = move_
            .names
            .unwrap_or_default()
            .find_by_lang(lang)
            .with_context(|| format!("No {} name for move {:?}", lang, move_.name))?;

        let mut by_level = HashMap::new();
        let mut by_machine = HashMap::new();

        for version_group_detail in &pokemon_move.version_group_details {
            match version_group_detail {}
        }

        Ok(Self {
            by_level,
            by_machine,
        })
    }
}
