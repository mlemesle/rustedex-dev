use std::path::PathBuf;

use clap::Parser;

/// Command line arguments
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// If set, Rustedex will spawn a web server to serve the application.
    #[arg(short, long)]
    pub serve: bool,

    /// Tells if Rustedex should generate the static files.
    #[arg(short, long)]
    pub generate: bool,

    #[arg(short, long, default_value = "./rustedex-dev")]
    pub path: PathBuf,
}
