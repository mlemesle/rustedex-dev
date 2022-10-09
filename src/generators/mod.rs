use anyhow::Result;
use handlebars::Handlebars;
use rustemon::client::RustemonClient;
use serde::Serialize;

use std::path::PathBuf;

mod all_pokemon;

pub async fn generate(
    mut base_path: PathBuf,
    hb: &Handlebars<'_>,
    rc: &RustemonClient,
) -> Result<()> {
    let pokemon_names = generate_pokemon_list(rc).await?;
    println!("{} Pokemons found, generating pages", pokemon_names.len());

    base_path.push("all_pokemon.html");

    all_pokemon::generate_all_pokemon_page(base_path, hb, pokemon_names).await?;

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
