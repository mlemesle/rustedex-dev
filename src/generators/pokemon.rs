use std::path::PathBuf;

use anyhow::{Context, Result};
use handlebars::Handlebars;
use rustemon::{
    client::RustemonClient,
    model::resource::{Name, NamedApiResource},
};
use serde::Serialize;

use super::render_to_write;
use crate::utils::{self, FindWordingByLang};

#[derive(Serialize)]
struct PokemonContext {
    artwork_url: String,
    name_fr: String,
    name_en: String,
    name_jp: String,
    types: Vec<String>,
    genus: String,
}

pub(super) async fn generate_pokemon_page(
    mut path: PathBuf,
    hb: &Handlebars<'_>,
    rc: &RustemonClient,
    pokemon: &str,
) -> Result<()> {
    path.push(format!("{}.html", pokemon));

    let pokemon = rustemon::pokemon::pokemon::get_by_name(pokemon, rc).await?;
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

    let pokemon_context = &PokemonContext {
        artwork_url,
        name_fr,
        name_en,
        name_jp: format!("{} {}", name_ja, name_roomaji),
        types,
        genus,
    };

    render_to_write(hb, "pokemon", pokemon_context, &path).await
}
