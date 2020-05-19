use std::path::PathBuf;
use structopt::{clap::ArgGroup, StructOpt};

fn is_fen(s: String) -> Result<(), String> {
    use fen::BoardState;

    BoardState::from_fen(&s)
        .map(|_| ())
        .map_err(|e| format!("{:?}", e))
    // TODO add proper error printing to fen
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "golemate",
    author = "Marcin Mielniczuk <marmistrz.dev@zoho.eu>",
    about = "Chess position solver using gWASM"
)]
#[structopt(group = ArgGroup::with_name("binary").required(true))]
pub(crate) struct Opts {
    #[structopt(short, long, validator = is_fen)]
    pub fen: String,
    #[structopt(short, long, help = "search depth")]
    pub depth: u32,
    #[structopt(
        short = "e",
        long = "engine",
        help = "path to a local engine to be used instead of gWASM",
        group = "binary"
    )]
    pub engine: Option<PathBuf>,
    #[structopt(
        short,
        long = "wasm",
        help = "path to the WASM part of the gWASM binary",
        requires = "js-path",
        requires = "workspace",
        group = "binary"
    )]
    pub wasm_path: Option<PathBuf>,
    #[structopt(short, long = "js", help = "path to the JS part of the gWASM binary")]
    pub js_path: Option<PathBuf>,
    #[structopt(long)]
    pub workspace: Option<PathBuf>,
}
