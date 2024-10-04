use clap::{Parser, Subcommand};
use color_eyre::Result;
use log::info;
use repo_root::{ProjectTypes, RepoRoot};
use std::{env, path::PathBuf};

#[derive(Debug, Parser)]
#[command(version, about, long_about=None)]
struct Cli {
    path: Option<PathBuf>,

    #[clap(short, long, default_value_t)]
    r#type: ProjectTypes,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    env_logger::init();

    let args = Cli::parse();
    let path = match args.path {
        Some(path) => path,
        None => env::current_dir()?,
    };

    let r#type = args.r#type;
    let Some(root) = r#type.find(&path)? else {
        eprintln!("No {} root found", r#type);
        std::process::exit(1);
    };

    let root = std::fs::canonicalize(root)?;
    println!("{}", root.display());
    Ok(())
}
