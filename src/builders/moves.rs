use std::collections::HashMap;

use anyhow::{Context, Result};
use async_trait::async_trait;
use rustemon::{
    client::RustemonClient,
    model::{
        moves::Move,
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
    moves_learned_by_machine_by_version_group: HashMap<String, Vec<MoveByMachine>>,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct MoveByLevel {
    name: String,
    lvl_learned_at: u8,
    type_: String,
    category: String,
    power: u8,
    accuracy: u8,
    pp: u8,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct MoveByMachine {
    name: String,
    machine_name: String,
    type_: String,
    category: String,
    power: u8,
    accuracy: u8,
    pp: u8,
}

struct MoveBuild {
    by_level: HashMap<String, Vec<MoveByLevel>>,
    by_machine: HashMap<String, Vec<MoveByMachine>>,
}

#[async_trait]
impl Builder<String> for Moves {
    async fn build(id: &String, rc: &RustemonClient, lang: &str) -> Result<Self> {
        let pokemon = rustemon::pokemon::pokemon::get_by_name(id, rc).await?;
        let pokemon_name = pokemon
            .species
            .follow(rc)
            .await?
            .names
            .find_by_lang(lang)
            .with_context(|| format!("No {} name for {}", lang, id))?;

        let mut moves_learned_by_level_by_version_group = HashMap::new();
        let mut moves_learned_by_machine_by_version_group = HashMap::new();

        for pokemon_move in &pokemon.moves {
            let MoveBuild {
                by_level,
                by_machine,
            } = MoveBuild::build(pokemon_move, rc, lang).await?;
            utils::fuse_maps_in_place(&mut moves_learned_by_level_by_version_group, by_level);
            utils::fuse_maps_in_place(&mut moves_learned_by_machine_by_version_group, by_machine);
        }

        moves_learned_by_level_by_version_group
            .values_mut()
            .for_each(|moves_by_level| {
                moves_by_level.sort_by_key(|move_by_level| move_by_level.lvl_learned_at)
            });
        moves_learned_by_machine_by_version_group
            .values_mut()
            .for_each(|moves_by_machine| {
                moves_by_machine.sort_by(|move_by_machine1, move_by_machine2| {
                    move_by_machine1
                        .machine_name
                        .cmp(&move_by_machine2.machine_name)
                })
            });
        let version_group_id_and_names = utils::get_version_group_id_and_names(
            moves_learned_by_level_by_version_group.keys().collect(),
        );

        Ok(Self {
            pokemon_name,
            version_group_id_and_names,
            moves_learned_by_level_by_version_group,
            moves_learned_by_machine_by_version_group,
        })
    }
}

#[async_trait]
impl Builder<PokemonMove> for MoveBuild {
    async fn build(pokemon_move: &PokemonMove, rc: &RustemonClient, lang: &str) -> Result<Self> {
        let move_ = pokemon_move.move_.follow(rc).await?;

        let move_name = move_
            .names
            .find_by_lang(lang)
            .with_context(|| format!("No {} name for move {:?}", lang, move_.name))?;

        let mut by_level = HashMap::new();
        let mut by_machine = HashMap::new();
        for PokemonMoveVersion {
            move_learn_method: NamedApiResource { name: mlm_name, .. },
            version_group: NamedApiResource { name: vg_name, .. },
            level_learned_at,
        } in &pokemon_move.version_group_details
        {
            match mlm_name.as_str() {
                "level-up" => by_level
                    .entry(vg_name.clone())
                    .or_insert_with(Vec::new)
                    .push(build_move_by_level(&move_name, *level_learned_at, &move_)?),
                "machine" => by_machine
                    .entry(vg_name.clone())
                    .or_insert_with(Vec::new)
                    .push(build_move_by_machine(&move_name, &move_, vg_name, rc, lang).await?),
                _ => (),
            }
        }

        Ok(Self {
            by_level,
            by_machine,
        })
    }
}

fn build_move_by_level(
    move_name: &str,
    level_learned_at: i64,
    move_: &Move,
) -> Result<MoveByLevel> {
    Ok(MoveByLevel {
        name: move_name.to_owned(),
        lvl_learned_at: level_learned_at as u8,
        type_: move_.type_.name.clone(),
        category: move_.damage_class.name.clone(),
        power: move_.power.unwrap_or_default() as u8,
        accuracy: move_.accuracy.unwrap_or_default() as u8,
        pp: move_.pp.unwrap_or_default() as u8,
    })
}

async fn build_move_by_machine(
    move_name: &str,
    move_: &Move,
    version_group: &String,
    rc: &RustemonClient,
    lang: &str,
) -> Result<MoveByMachine> {
    let machine_name = move_
        .machines
        .iter()
        .find(|machine_by_version_detail| {
            &machine_by_version_detail.version_group.name == version_group
        })
        .unwrap()
        .machine
        .follow(rc)
        .await?
        .item
        .follow(rc)
        .await?
        .names
        .find_by_lang(lang)
        .unwrap();

    Ok(MoveByMachine {
        name: move_name.to_owned(),
        machine_name,
        type_: move_.type_.name.clone(),
        category: move_.damage_class.name.clone(),
        power: move_.power.unwrap_or_default() as u8,
        accuracy: move_.accuracy.unwrap_or_default() as u8,
        pp: move_.pp.unwrap_or_default() as u8,
    })
}
