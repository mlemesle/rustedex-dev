use anyhow::{Context, Result};
use async_trait::async_trait;
use rustemon::{
    client::RustemonClient,
    model::resource::{Name, NamedApiResource},
};
use serde::Serialize;

use super::Builder;
use crate::utils::{self, FindWordingByLang};

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
}

#[async_trait]
impl Builder for Card {
    async fn build(id: &str, rc: &RustemonClient) -> Result<Self> {
        let pokemon = rustemon::pokemon::pokemon::get_by_name(id, rc).await?;
        let pokemon_specie = pokemon.species.unwrap().follow(rc).await?;
        let growth_rate = pokemon_specie.growth_rate.unwrap().follow(rc).await?;

        let artwork_url = pokemon
            .sprites
            .with_context(|| format!("No sprites for {:?}", pokemon.name))?
            .other
            .with_context(|| format!("No 'other' sprite for {:?}", pokemon.name))?
            .official_artwork
            .with_context(|| format!("No official artwork for {:?}", pokemon.name))?
            .front_default
            .with_context(|| format!("No 'front_default' sprite for {:?}", pokemon.name))?;

        let mut name_fr = "".to_string();
        let mut name_en = "".to_string();
        let mut name_ja = "".to_string();
        let mut name_roomaji = "".to_string();

        for name in pokemon_specie.names.unwrap_or_default() {
            if let Name {
                name: Some(n),
                language: Some(NamedApiResource { name: Some(l), .. }),
            } = name
            {
                match l.as_ref() {
                    "fr" => name_fr = n,
                    "en" => name_en = n,
                    "ja" => name_ja = n,
                    "roomaji" => name_roomaji = n,
                    _ => (),
                }
            }
        }

        let types = pokemon
            .types
            .unwrap_or_default()
            .into_iter()
            .map(|pokemon_type| pokemon_type.type_.unwrap().name.unwrap())
            .collect();

        let genus = pokemon_specie
            .genera
            .unwrap_or_default()
            .find_by_lang(utils::EN)
            .with_context(|| format!("No genus found for {:?}", pokemon_specie.name))?;

        let height = pokemon
            .height
            .with_context(|| format!("No height for {:?}", pokemon.name))?
            as f32
            / 10.0;

        let weight = pokemon
            .weight
            .with_context(|| format!("No weight for {:?}", pokemon.name))?
            as f32
            / 10.0;

        let card = Card {
            artwork_url,
            name_fr,
            name_en,
            name_jp: format!("{} {}", name_ja, name_roomaji),
            types,
            genus,
            height,
            weight,
        };

        Ok(card)
    }
}
