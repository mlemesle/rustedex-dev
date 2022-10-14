#![recursion_limit = "256"]

use std::{
    fs::{create_dir_all, remove_dir_all, DirBuilder, File},
    io::Write,
    path::{Path, PathBuf},
};

use anyhow::Result;
use clap::Parser;
use include_dir::DirEntry;
use rustemon::client::RustemonClient;
use warp::Filter;

mod args;
mod builders;
mod find_by_lang;
mod generators;
mod handlebars;
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

fn init_rustemon_client() -> RustemonClient {
    RustemonClient::default()
}

async fn run(base_path: PathBuf) {
    let route = warp::get().and(warp::fs::dir(base_path));

    warp::serve(route).run(([0, 0, 0, 0], 3030)).await;
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = args::Args::parse();
    let path = PathBuf::from("./rustedex");
    println!(
        "{}",
        match remove_dir_all(&path) {
            Ok(_) => "Clean successful",
            Err(_) => "Nothing to clean",
        }
    );
    DirBuilder::new().create(&path)?;
    DirBuilder::new().create(path.join("pokemons"))?;
    export_assets(&path)?;

    if args.generate {
        let hb = handlebars::init_handlebars()?;
        let rc = init_rustemon_client();

        generators::generate(path.clone(), &hb, &rc).await?;
    }

    if args.serve {
        run(path).await;
    }

    Ok(())
}
