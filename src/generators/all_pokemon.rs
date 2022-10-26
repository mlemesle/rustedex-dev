use std::path::PathBuf;

use crate::{
    builders::{all_pokemon::AllPokemon, Builder},
    context::Context,
};

use super::render_to_write;

use anyhow::Result;

pub(super) async fn generate_all_pokemon_page(
    mut path: PathBuf,
    pokemon_id_and_path: &Vec<(String, PathBuf)>,
    context: &Context<'_>,
) -> Result<()> {
    let all_pokemon = &AllPokemon::build(pokemon_id_and_path, context.rc(), context.lang()).await?;
    path.push("all_pokemon.html");
    render_to_write(context.hb(), "all_pokemon", all_pokemon, &path).await
}
