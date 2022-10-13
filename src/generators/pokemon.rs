use std::path::PathBuf;

use anyhow::Result;
use handlebars::Handlebars;
use rustemon::client::RustemonClient;

use crate::builders::{pokemon::Pokemon, Builder};

use super::render_to_write;

pub(super) async fn generate_pokemon_page(
    mut path: PathBuf,
    hb: &Handlebars<'_>,
    rc: &RustemonClient,
    pokemon_id: &str,
) -> Result<()> {
    path.push(format!("{}.html", pokemon_id));
    let pokemon = Pokemon::build(pokemon_id, rc).await?;
    render_to_write(hb, "pokemon", &pokemon, &path).await
}
