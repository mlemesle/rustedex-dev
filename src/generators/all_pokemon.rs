use std::path::PathBuf;

use anyhow::Result;
use handlebars::Handlebars;
use serde::Serialize;

use super::render_to_write;

#[derive(Serialize)]
struct AllPokemonContext<'a> {
    pokemon_names: &'a Vec<String>,
}

pub(super) async fn generate_all_pokemon_page(
    path: PathBuf,
    hb: &Handlebars<'_>,
    pokemon_names: &Vec<String>,
) -> Result<()> {
    let all_pokemon_context = &AllPokemonContext { pokemon_names };

    render_to_write(hb, "all_pokemon", all_pokemon_context, &path).await
}
