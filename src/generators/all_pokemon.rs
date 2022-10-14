use std::path::PathBuf;

use anyhow::Result;
use handlebars::Handlebars;
use rustemon::client::RustemonClient;

use super::render_to_write;
use crate::builders::{all_pokemon::AllPokemon, Builder};

pub(super) async fn generate_all_pokemon_page(
    mut path: PathBuf,
    hb: &Handlebars<'_>,
    rc: &RustemonClient,
    pokemon_names_and_paths: Vec<(String, PathBuf)>,
) -> Result<()> {
    let all_pokemon = &AllPokemon::build(pokemon_names_and_paths, rc).await?;
    path.push("index.html");
    render_to_write(hb, "all_pokemon", all_pokemon, &path).await
}
