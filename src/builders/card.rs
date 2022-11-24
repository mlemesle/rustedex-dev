use std::collections::HashMap;

use anyhow::{Context, Result};
use async_trait::async_trait;
use rustemon::{
    client::RustemonClient,
    model::resource::{Name, NamedApiResource},
    Follow,
};
use serde::Serialize;

use super::Builder;
use crate::{
    find_by_lang::FindWordingByLang,
    utils::{
        get_abilities_names_by_lang, get_effort_points_map_by_lang, get_egg_groups_names_by_lang,
    },
};

#[derive(Serialize)]
pub(crate) struct GenderRates {
    male: f32,
    female: f32,
}

#[derive(Serialize)]
pub(crate) struct Card {
    artwork_url: String,
    name_fr: String,
    name_en: String,
    name_jp: String,
    types: Vec<String>,
    genus: String,
    height: f32,
    weight: f32,
    abilities: Vec<String>,
    egg_groups: Vec<String>,
    steps_until_hatch: i64,
    effort_points: HashMap<String, i64>,
    base_experience: i64,
    lvl_100_experience: i64,
    gender_rates: Option<GenderRates>,
    color: String,
    capture_rate: i64,
}

#[async_trait]
impl Builder<String> for Card {
    async fn build(id: &String, rc: &RustemonClient, lang: &str) -> Result<Self> {
        let pokemon = rustemon::pokemon::pokemon::get_by_name(id, rc).await?;
        let pokemon_specie = pokemon.species.follow(rc).await?;
        let growth_rate = pokemon_specie.growth_rate.follow(rc).await?;
        let pokemon_color = pokemon_specie.color.follow(rc).await?;

        let artwork_url = pokemon
            .sprites
            .other
            .official_artwork
            .front_default
            .unwrap_or_else(|| "https://media.giphy.com/media/DRfu7BT8ZK1uo/giphy.gif".into());

        let mut name_fr = "".to_string();
        let mut name_en = "".to_string();
        let mut name_ja = "".to_string();
        let mut name_roomaji = "".to_string();

        for Name {
            name: n,
            language: NamedApiResource { name: l, .. },
        } in pokemon_specie.names
        {
            match l.as_ref() {
                "fr" => name_fr = n,
                "en" => name_en = n,
                "ja" => name_ja = n,
                "roomaji" => name_roomaji = n,
                _ => (),
            }
        }

        let types = pokemon
            .types
            .into_iter()
            .map(|pokemon_type| pokemon_type.type_.name)
            .collect();

        let genus = pokemon_specie
            .genera
            .find_by_lang(lang)
            .with_context(|| format!("No genus found for {:?}", pokemon_specie.name))?;

        let height = pokemon.height as f32 / 10.0;

        let weight = pokemon.weight as f32 / 10.0;

        let abilities = get_abilities_names_by_lang(pokemon.abilities, lang, rc).await?;

        let egg_groups = get_egg_groups_names_by_lang(pokemon_specie.egg_groups, lang, rc).await?;

        let steps_until_hatch = (pokemon_specie.hatch_counter.unwrap_or_default() + 1) * 255;

        let effort_points = get_effort_points_map_by_lang(pokemon.stats, lang, rc).await?;

        let base_experience = pokemon.base_experience.unwrap_or_default();

        let lvl_100_experience = growth_rate
            .levels
            .iter()
            .find(|level| level.level == 100)
            .map(|level_100| level_100.experience)
            .with_context(|| format!("No level 100 experience for {:?}", pokemon.name))?;

        let female_rate = pokemon_specie.gender_rate;
        let gender_rates = if female_rate == -1 {
            None
        } else {
            let female_rate = female_rate as f32 * 12.5;
            Some(GenderRates {
                female: female_rate,
                male: 100.0 - female_rate,
            })
        };

        let color = pokemon_color
            .names
            .find_by_lang(lang)
            .with_context(|| format!("No color in {} for {:?}", lang, pokemon_color.name))?;

        let capture_rate = pokemon_specie.capture_rate;

        let card = Card {
            artwork_url,
            name_fr,
            name_en,
            name_jp: format!("{} {}", name_ja, name_roomaji),
            types,
            genus,
            height,
            weight,
            abilities,
            egg_groups,
            steps_until_hatch,
            effort_points,
            base_experience,
            lvl_100_experience,
            gender_rates,
            color,
            capture_rate,
        };

        Ok(card)
    }
}
