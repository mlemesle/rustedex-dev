use std::collections::HashMap;

use anyhow::{Context, Result};
use rustemon::{
    client::RustemonClient,
    model::{
        pokemon::{EggGroup, PokemonAbility, PokemonStat},
        resource::NamedApiResource,
    },
    Follow,
};

use crate::find_by_lang::FindWordingByLang;

pub(crate) async fn get_abilities_names_by_lang(
    abilities: Vec<PokemonAbility>,
    lang: &str,
    rc: &RustemonClient,
) -> Result<Vec<String>> {
    let mut result = Vec::with_capacity(abilities.len());

    for ability in abilities {
        let ability_name = ability
            .ability
            .unwrap()
            .follow(rc)
            .await?
            .names
            .unwrap_or_default()
            .find_by_lang(lang)
            .unwrap();
        result.push(ability_name);
    }

    Ok(result)
}

pub(crate) async fn get_egg_groups_names_by_lang(
    egg_groups: Vec<NamedApiResource<EggGroup>>,
    lang: &str,
    rc: &RustemonClient,
) -> Result<Vec<String>> {
    let mut result = Vec::with_capacity(egg_groups.len());

    for egg_group in egg_groups {
        let egg_group_name = egg_group
            .follow(rc)
            .await?
            .names
            .unwrap_or_default()
            .find_by_lang(lang)
            .with_context(|| format!("No {} for {:?}", lang, egg_group))?;
        result.push(egg_group_name);
    }

    Ok(result)
}

pub(crate) async fn get_effort_points_map_by_lang(
    pokemon_stats: Vec<PokemonStat>,
    lang: &str,
    rc: &RustemonClient,
) -> Result<HashMap<String, i64>> {
    let mut result = HashMap::new();

    for pokemon_stat in pokemon_stats {
        let effort_points = pokemon_stat.effort.unwrap_or_default();
        if effort_points > 0 {
            let stat_name = pokemon_stat
                .stat
                .as_ref()
                .with_context(|| format!("No stat for {:?}", pokemon_stat))?
                .follow(rc)
                .await?
                .names
                .unwrap_or_default()
                .find_by_lang(lang)
                .with_context(|| format!("No name in {} for {:?}", lang, pokemon_stat))?;
            result.insert(stat_name, effort_points);
        }
    }

    Ok(result)
}
