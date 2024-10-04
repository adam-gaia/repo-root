use clap::{Parser, Subcommand};
use color_eyre::Result;
use log::info;
use repo_root::{ProjectType, RepoRoot, TraversalDirection};
use std::{
    env,
    path::{Path, PathBuf},
};

pub struct FooProject {}
impl ProjectType for FooProject {
    fn direction() -> TraversalDirection {
        TraversalDirection::Backwards
    }

    /// Look for a '.foo' file
    fn condition(path: &Path) -> bool {
        let dockerfile = path.join(".foo");
        dockerfile.is_file()
    }
}

#[derive(Debug, Parser)]
#[command(version, about, long_about=None)]
struct Cli {
    path: Option<PathBuf>,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    env_logger::init();

    let args = Cli::parse();
    let path = match args.path {
        Some(path) => path,
        None => env::current_dir()?,
    };

    let Some(root) = RepoRoot::<FooProject>::find(&path)? else {
        eprintln!("No '.foo' found in cwd ancestry");
        std::process::exit(1);
    };

    let root_path = root.path();
    let root_path = std::fs::canonicalize(root_path)?;
    println!("{}", root_path.display());
    Ok(())
}
