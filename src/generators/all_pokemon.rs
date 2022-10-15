use std::path::PathBuf;

use super::render_to_write;
use crate::{
    builders::{all_pokemon::AllPokemon, Builder},
    context::Context,
};

use anyhow::Result;

pub(super) async fn generate_all_pokemon_page(
    mut path: PathBuf,
    pokemon_id_and_names_and_paths: Vec<(i64, String, PathBuf)>,
    context: &Context<'_>,
) -> Result<()> {
    let all_pokemon =
        &AllPokemon::build(pokemon_id_and_names_and_paths, context.rc(), context.lang()).await?;
    path.push("index.html");
    render_to_write(context.hb(), "all_pokemon", all_pokemon, &path).await
}
