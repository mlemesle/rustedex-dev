use std::path::PathBuf;

use crate::{
    builders::{pokemon::Pokemon, Builder},
    context::Context as CContext,
    find_by_lang::FindWordingByLang,
};
use anyhow::{Context, Result};

use super::render_to_write;

pub(super) async fn generate_pokemon_page(
    mut path: PathBuf,
    pokemon_id: String,
    context: &CContext<'_>,
) -> Result<(String, PathBuf)> {
    let relative_path = PathBuf::from(format!("pokemons/{}.html", pokemon_id));
    path.push(&relative_path);

    let pokemon = Pokemon::build(pokemon_id.clone(), context.rc(), context.lang()).await?;

    render_to_write(context.hb(), "pokemon", &pokemon, &path).await?;

    let pokemon_name = rustemon::pokemon::pokemon::get_by_name(&pokemon_id, context.rc())
        .await?
        .species
        .with_context(|| format!("No species for {}", pokemon_id))?
        .follow(context.rc())
        .await?
        .names
        .unwrap_or_default()
        .find_by_lang(context.lang())
        .with_context(|| format!("No {} name for {}", context.lang(), pokemon_id))?;

    Ok((pokemon_name, relative_path))
}
