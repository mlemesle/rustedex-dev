use std::path::PathBuf;

use anyhow::{Context, Result};
use handlebars::Handlebars;
use rustemon::client::RustemonClient;

use crate::{
    builders::{pokemon::Pokemon, Builder},
    find_by_lang::{self, FindWordingByLang},
};

use super::render_to_write;

pub(super) async fn generate_pokemon_page(
    mut path: PathBuf,
    pokemon_id: String,
    hb: &Handlebars<'_>,
    rc: &RustemonClient,
) -> Result<(String, PathBuf)> {
    let relative_path = PathBuf::from(format!("pokemons/{}.html", pokemon_id));
    path.push(&relative_path);
    let pokemon = Pokemon::build(pokemon_id.clone(), rc).await?;
    render_to_write(hb, "pokemon", &pokemon, &path).await?;

    let pokemon_name = rustemon::pokemon::pokemon::get_by_name(&pokemon_id, rc)
        .await?
        .species
        .with_context(|| format!("No species for {}", pokemon_id))?
        .follow(rc)
        .await?
        .names
        .unwrap_or_default()
        .find_by_lang(find_by_lang::FR)
        .with_context(|| format!("No {} name for {}", find_by_lang::FR, pokemon_id))?;

    Ok((pokemon_name, relative_path))
}
