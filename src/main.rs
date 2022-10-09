use std::{fs::DirBuilder, path::PathBuf};

use anyhow::Result;
use clap::Parser;
use handlebars::Handlebars;
use rustemon::client::RustemonClient;
use warp::Filter;

mod args;
mod generators;

fn init_handlebars() -> Result<Handlebars<'static>> {
    let mut hb = Handlebars::new();
    hb.set_strict_mode(true);

    hb.register_templates_directory(".hbs", "templates/")?;

    Ok(hb)
}

fn init_rustemon_client() -> RustemonClient {
    RustemonClient::default()
}

async fn run(base_path: PathBuf) {
    let route = warp::path("rustedex").and(warp::fs::dir(base_path));

    warp::serve(route).run(([127, 0, 0, 1], 3030)).await;
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = args::Args::parse();
    let path = PathBuf::from("./rustedex");
    DirBuilder::new().recursive(true).create(&path)?;

    if args.generate {
        let hb = init_handlebars()?;
        let rc = init_rustemon_client();

        generators::generate(path.clone(), &hb, &rc).await?;
    }

    if args.serve {
        run(path).await;
    }

    Ok(())
}
