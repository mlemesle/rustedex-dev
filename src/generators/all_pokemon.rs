use std::path::PathBuf;

use anyhow::Result;
use handlebars::Handlebars;
use serde_json::json;

pub(super) async fn generate_pokemon_list(
    path: PathBuf,
    hb: &Handlebars<'_>,
    pokemon_names: Vec<String>,
) -> Result<()> {
    let mut all_pokemon_page = std::fs::File::create(path)?;

    hb.render_to_write(
        "base",
        &json!({ "pokemon_names": pokemon_names }),
        &mut all_pokemon_page,
    )?;

    Ok(())
}
