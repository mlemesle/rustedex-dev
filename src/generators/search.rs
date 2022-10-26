use std::path::PathBuf;

use super::render_to_write;
use crate::{
    builders::{search::Search, Builder},
    context::Context,
};

use anyhow::Result;

pub(super) async fn generate_search_page(
    mut path: PathBuf,
    pokemon_id_and_names_and_paths: &Vec<(String, PathBuf)>,
    context: &Context<'_>,
) -> Result<()> {
    let search =
        &Search::build(pokemon_id_and_names_and_paths, context.rc(), context.lang()).await?;
    path.push("search.html");
    render_to_write(context.hb(), "search", search, &path).await
}
