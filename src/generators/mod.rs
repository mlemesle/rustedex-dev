use anyhow::Result;
use handlebars::Handlebars;
use indicatif::ProgressBar;
use rustemon::client::RustemonClient;
use serde::Serialize;

use std::path::PathBuf;

use crate::context::Context;

mod pokemon;
mod search;

pub(crate) async fn generate(base_path: PathBuf, context: &Context<'_>) -> Result<()> {
    let pokemon_names = generate_pokemon_list(context.rc()).await?;

    let mut generated_pokemons = Vec::with_capacity(pokemon_names.len());
    println!("{} Pokemons found, generating pages", pokemon_names.len());
    let pg = ProgressBar::new(pokemon_names.len() as u64);
    for pokemon_name in pokemon_names {
        generated_pokemons
            .push(pokemon::generate_pokemon_page(base_path.clone(), pokemon_name, context).await?);
        pg.inc(1);
    }

    search::generate_search_page(base_path, generated_pokemons, context).await?;

    Ok(())
}

async fn generate_pokemon_list(rc: &RustemonClient) -> Result<Vec<String>> {
    let nb_pokemon = rustemon::pokemon::pokemon::get_page(rc)
        .await?
        .count
        .unwrap_or_default();

    let mut pokemon_names = Vec::with_capacity(nb_pokemon as usize);

    let mut offset = 0;
    while offset < nb_pokemon {
        let page = rustemon::pokemon::pokemon::get_page_with_param(offset, 100, rc).await?;

        page.results
            .unwrap_or_default()
            .into_iter()
            .for_each(|pokemon_name| pokemon_names.push(pokemon_name.name.unwrap()));
        offset += 100;
    }

    #[cfg(debug_assertions)]
    pokemon_names.truncate(10);

    Ok(pokemon_names)
}

#[derive(Serialize)]
struct BaseContext<'a, T: Serialize> {
    inner_template: &'a str,
    data: T,
}

pub(self) async fn render_to_write<T>(
    hb: &Handlebars<'_>,
    inner_template: &str,
    data: &T,
    file_path: &PathBuf,
) -> Result<()>
where
    T: Serialize,
{
    let mut file = std::fs::File::create(file_path)?;
    let context = &BaseContext {
        inner_template,
        data,
    };

    hb.render_to_write("base", context, &mut file)?;

    Ok(())
}
