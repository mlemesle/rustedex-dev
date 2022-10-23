#![recursion_limit = "256"]

use std::{
    fs::{create_dir_all, remove_dir_all, DirBuilder, File},
    io::Write,
    path::{Path, PathBuf},
};

use anyhow::Result;
use clap::Parser;
use include_dir::DirEntry;
use warp::Filter;

mod args;
mod builders;
mod context;
mod find_by_lang;
mod generators;
mod utils;

const ASSETS: include_dir::Dir = include_dir::include_dir!("./assets");

fn export_assets(path: &Path) -> Result<()> {
    let assets = ASSETS.entries();
    inner_export_assets(&path.join("assets"), assets)
}

fn inner_export_assets(path: &Path, entries: &[DirEntry]) -> Result<()> {
    for entry in entries {
        match entry {
            DirEntry::File(file) => {
                let target_file_path = path.join(file.path());
                let file_parent_dir = target_file_path
                    .parent()
                    .unwrap_or_else(|| Path::new("./rustedex"));
                if !file_parent_dir.exists() {
                    create_dir_all(file_parent_dir)?;
                }
                let mut file_to = File::create(target_file_path)?;
                file_to.write_all(file.contents())?;
            }
            DirEntry::Dir(dir) => {
                inner_export_assets(path, dir.entries())?;
            }
        }
    }

    Ok(())
}

async fn run(base_path: PathBuf) {
    let route = warp::path("rustedex").and(warp::fs::dir(base_path));

    warp::serve(route).run(([0, 0, 0, 0], 3030)).await;
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = args::Args::parse();

    println!(
        "{}",
        match remove_dir_all(&args.path) {
            Ok(_) => "Clean successful",
            Err(_) => "Nothing to clean",
        }
    );
    DirBuilder::new().create(&args.path)?;
    DirBuilder::new().create(args.path.join("pokemons"))?;
    export_assets(&args.path)?;

    if args.generate {
        let context = context::Context::try_new()?;
        generators::generate(args.path.clone(), &context).await?;
        println!("Static file generated at {}", args.path.display());
    }

    if args.serve {
        println!("Starting server at http://localhost:3030/rustedex/search.html");
        run(args.path).await;
    }

    Ok(())
}
