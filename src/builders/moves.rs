use std::collections::HashMap;

use anyhow::{Context, Result};
use async_trait::async_trait;
use rustemon::{
    client::RustemonClient,
    model::{
        pokemon::{PokemonMove, PokemonMoveVersion},
        resource::NamedApiResource,
    },
    Follow,
};
use serde::{Deserialize, Serialize};

use crate::{find_by_lang::FindWordingByLang, utils};

use super::Builder;

#[derive(Serialize)]
pub(crate) struct Moves {
    pokemon_name: String,
    version_group_id_and_names: Vec<(&'static str, &'static str)>,
    moves_learned_by_level_by_version_group: HashMap<String, Vec<MoveByLevel>>,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct MoveByLevel {
    name: String,
    type_: String,
    lvl_learned_at: u8,
}

struct MoveBuild {
    by_level: HashMap<String, Vec<MoveByLevel>>,
    // by_machine: HashMap<String, Vec<Move>>,
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

        let mut moves_learned_by_level_by_version_group = match pokemon.moves {
            Some(pokemon_moves) => {
                let mut moves_learned_by_level_by_version_group: HashMap<String, Vec<MoveByLevel>> =
                    HashMap::new();

                for pokemon_move in &pokemon_moves {
                    let move_build = MoveBuild::build(pokemon_move, rc, lang).await?;
                    utils::fuse_maps_in_place(
                        &mut moves_learned_by_level_by_version_group,
                        move_build.by_level,
                    );
                }

                moves_learned_by_level_by_version_group
            }
            None => HashMap::new(),
        };

        moves_learned_by_level_by_version_group
            .values_mut()
            .for_each(|moves_by_level| {
                moves_by_level.sort_by_key(|move_by_level| move_by_level.lvl_learned_at)
            });
        let version_group_id_and_names = utils::get_version_group_id_and_names(
            moves_learned_by_level_by_version_group
                .keys()
                .into_iter()
                .collect(),
        );

        Ok(Self {
            pokemon_name,
            version_group_id_and_names,
            moves_learned_by_level_by_version_group,
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
        // let mut by_machine = HashMap::new();
        for version_group_detail in pokemon_move.version_group_details.as_ref().unwrap() {
            if let PokemonMoveVersion {
                move_learn_method:
                    Some(NamedApiResource {
                        name: Some(mlm_name),
                        ..
                    }),
                version_group:
                    Some(NamedApiResource {
                        name: Some(vg_name),
                        ..
                    }),
                level_learned_at: Some(level_learned_at),
            } = version_group_detail
            {
                match mlm_name.as_str() {
                    "level-up" => by_level
                        .entry(vg_name.clone())
                        .or_insert_with(|| Vec::new())
                        .push(MoveByLevel {
                            name: move_name.clone(),
                            type_: move_
                                .type_
                                .as_ref()
                                .with_context(|| format!("No type for move {}", move_name))?
                                .name
                                .as_ref()
                                .unwrap()
                                .clone(),
                            lvl_learned_at: *level_learned_at as u8,
                        }),
                    _ => (),
                }
            }
        }

        Ok(Self {
            by_level,
            // by_machine,
        })
    }
}
