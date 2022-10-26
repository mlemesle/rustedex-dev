use std::path::PathBuf;

use crate::{
    builders::{pokemon::Pokemon, Builder},
    context::Context,
};
use anyhow::Result;

use super::render_to_write;

pub(super) async fn generate_pokemon_page(
    mut path: PathBuf,
    pokemon_id: String,
    context: &Context<'_>,
) -> Result<(String, PathBuf)> {
    let relative_path = PathBuf::from(format!("pokemons/{}.html", pokemon_id));
    path.push(&relative_path);

    let pokemon = Pokemon::build(&pokemon_id, context.rc(), context.lang()).await?;

    render_to_write(context.hb(), "pokemon", &pokemon, &path).await?;

    Ok((pokemon_id, relative_path))
}
