use std::{collections::HashMap, hash::Hash};

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

pub(crate) fn fuse_maps_in_place<K, V>(first: &mut HashMap<K, Vec<V>>, second: HashMap<K, Vec<V>>)
where
    K: Eq + Hash,
{
    second
        .into_iter()
        .for_each(|(k, v)| first.entry(k).or_insert_with(Vec::new).extend(v));
}

pub(crate) fn get_version_group_id_and_names(
    to_retain: Vec<&String>,
) -> Vec<(&'static str, &'static str)> {
    Vec::from([
        ("red-blue", "Red & Blue"),
        ("yellow", "Yellow"),
        ("gold-silver", "Gold & Silver"),
        ("crystal", "Crystal"),
        ("ruby-sapphire", "Ruby & Sapphire"),
        ("emerald", "Emerald"),
        ("firered-leafgreen", "Fire Red & Leaf Green"),
        ("diamond-pearl", "Diamond & Pearl"),
        ("platinum", "Platinum"),
        ("heartgold-soulsilver", "HeartGold & SoulSilver"),
        ("black-white", "Black & White"),
        ("colosseum", "Colosseum"),
        ("xd", "XD"),
        ("black-2-white-2", "Black 2 & White 2"),
        ("x-y", "X & Y"),
        ("omega-ruby-alpha-sapphire", "Omega Ruby & Alpha Sapphire"),
        ("sun-moon", "Sun & Moon"),
        ("ultra-sun-ultra-moon", "Ultra Sun & Ultra Moon"),
        (
            "lets-go-pikachu-lets-go-eevee",
            "Let's Go Pikachu & Let's Go Eevee",
        ),
        ("sword-shield", "Sword & Shield"),
        ("the-isle-of-armor", "The isle of Armor"),
        ("the-crown-tundra", "The crown Tundra"),
        (
            "brilliant-diamond-and-shining-pearl",
            "Brilliant Diamond & Shining Pearl",
        ),
        ("legends-arceus", "Legends Arceus"),
    ])
    .into_iter()
    .filter(|&elem| to_retain.contains(&&elem.0.to_string()))
    .collect()
}

pub(crate) fn get_type_ids() -> Vec<String> {
    vec![
        "normal".to_string(),
        "fighting".to_string(),
        "flying".to_string(),
        "poison".to_string(),
        "ground".to_string(),
        "rock".to_string(),
        "bug".to_string(),
        "ghost".to_string(),
        "steel".to_string(),
        "fire".to_string(),
        "water".to_string(),
        "grass".to_string(),
        "electric".to_string(),
        "psychic".to_string(),
        "ice".to_string(),
        "dragon".to_string(),
        "dark".to_string(),
        "fairy".to_string(),
    ]
}
