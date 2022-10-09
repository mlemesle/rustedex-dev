use std::path::PathBuf;

use anyhow::Result;
use handlebars::Handlebars;
use serde_json::json;

use super::render_to_write;

pub(super) async fn generate_pokemon_list(
    path: PathBuf,
    hb: &Handlebars<'_>,
    pokemon_names: Vec<String>,
) -> Result<()> {
    render_to_write(
        hb,
        "all_pokemon",
        &json!({ "pokemon_names": pokemon_names }),
        &path,
    )
    .await
}
