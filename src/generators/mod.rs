use anyhow::Result;
use handlebars::Handlebars;
use indicatif::ProgressBar;
use rustemon::{client::RustemonClient, Follow};
use serde::Serialize;

use std::path::PathBuf;

use crate::context::Context;

mod all_pokemon;
mod home;
mod pokemon;
mod search;

pub(crate) async fn generate(base_path: PathBuf, context: &Context<'_>) -> Result<()> {
    println!("Fetching PokeAPI to count Pokemons to generate");
    let pokemon_names = generate_pokemon_list(context.rc()).await?;
    println!("{} Pokemons found", pokemon_names.len());

    let mut generated_pokemons = Vec::with_capacity(pokemon_names.len());

    println!("Starting all pages generation");
    println!("Starting generation for Pokemons");
    let pg = ProgressBar::new(pokemon_names.len() as u64);
    for pokemon_name in &pokemon_names {
        pg.println(&format!("Generating page for {}", pokemon_name));
        generated_pokemons
            .push(pokemon::generate_pokemon_page(base_path.clone(), pokemon_name, context).await?);
        pg.println(&format!("Generated page for {}", pokemon_name));
        pg.inc(1);
    }
    println!("Pokemon pages generated");

    println!("Generating search page");
    search::generate_search_page(base_path.clone(), &generated_pokemons, context).await?;
    println!("Search page generated");

    println!("Generating all Pokemons page");
    all_pokemon::generate_all_pokemon_page(base_path.clone(), &generated_pokemons, context).await?;
    println!("All Pokemons page generated");

    println!("Generating home page");
    home::generate_home_page(base_path, context).await?;
    println!("Home page generated");

    Ok(())
}

async fn generate_pokemon_list(rc: &RustemonClient) -> Result<Vec<String>> {
    let nb_pokemon = rustemon::pokemon::pokemon::get_page(rc).await?.count;

    let mut pokemon_names = Vec::with_capacity(nb_pokemon as usize);

    let mut offset = 0;
    while offset < nb_pokemon {
        let page = rustemon::pokemon::pokemon::get_page_with_param(offset, 100, rc).await?;

        for p in page.results.into_iter() {
            if p.follow(rc).await?.is_default {
                pokemon_names.push(p.name);
            }
        }
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
