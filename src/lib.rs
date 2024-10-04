// TODO: take asref<path> instead of path for arguments
use color_eyre::eyre::bail;
use color_eyre::Result;
use log::debug;
use projects::{DockerProject, GitProject, NixProject, NodeProject, PythonProject, RustProject};
use std::env;
use std::fmt::Display;
use std::marker::PhantomData;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use winnow::combinator::alt;
use winnow::prelude::*;
use winnow::Parser;

pub mod projects;

/// Traverse a path's ancestry, walking the chain of parents to the root (/) until conditoin is met
fn traverse_backwards<F>(cwd: &Path, condition: F) -> Result<Option<PathBuf>>
where
    F: Fn(&Path) -> bool,
{
    let mut cwd = cwd.to_path_buf();
    loop {
        if condition(&cwd) {
            return Ok(Some(cwd.to_path_buf()));
        }

        let Some(parent) = cwd.parent() else {
            break;
        };
        cwd = parent.to_path_buf();
    }

    Ok(None)
}

/// Traverse a path, starting from the root (/) walking down the children to the full path until condition is met
fn traverse_forward<F>(cwd: &Path, condition: F) -> Result<Option<PathBuf>>
where
    F: Fn(&Path) -> bool,
{
    let mut path = PathBuf::new();
    for component in cwd.components() {
        path = path.join(component);
        if condition(&path) {
            return Ok(Some(path));
        }
    }

    Ok(None)
}

/// Do not go up/down the directory tree; only look in cwd
fn no_traversal<F>(cwd: &Path, condition: F) -> Result<Option<PathBuf>>
where
    F: Fn(&Path) -> bool,
{
    if condition(&cwd) {
        return Ok(Some(cwd.to_path_buf()));
    }

    Ok(None)
}

// TODO: support looking at env var PRJ_ROOT
// TODO: golang
// TODO: zig
// TODO: Cmake

#[derive(Debug, Copy, Clone)]
pub enum ProjectTypes {
    Git,
    Docker,
    NodeJS,
    Rust,
    Python,
    Nix,
}

impl ProjectTypes {
    pub fn find(&self, path: &Path) -> Result<Option<PathBuf>> {
        match self {
            ProjectTypes::Git => {
                let root: Option<RepoRoot<GitProject>> = RepoRoot::find(path)?;
                let root = root.map(|r| r.path());
                Ok(root)
            }

            ProjectTypes::Docker => {
                let root: Option<RepoRoot<DockerProject>> = RepoRoot::find(path)?;
                let root = root.map(|r| r.path());
                Ok(root)
            }

            ProjectTypes::NodeJS => {
                let root: Option<RepoRoot<NodeProject>> = RepoRoot::find(path)?;
                let root = root.map(|r| r.path());
                Ok(root)
            }

            ProjectTypes::Rust => {
                let root: Option<RepoRoot<RustProject>> = RepoRoot::find(path)?;
                let root = root.map(|r| r.path());
                Ok(root)
            }

            ProjectTypes::Python => {
                let root: Option<RepoRoot<PythonProject>> = RepoRoot::find(path)?;
                let root = root.map(|r| r.path());
                Ok(root)
            }

            ProjectTypes::Nix => {
                let root: Option<RepoRoot<NixProject>> = RepoRoot::find(path)?;
                let root = root.map(|r| r.path());
                Ok(root)
            }
        }
    }
}

fn git(s: &mut &str) -> PResult<ProjectTypes> {
    "git".map(|_| ProjectTypes::Git).parse_next(s)
}

fn docker(s: &mut &str) -> PResult<ProjectTypes> {
    "docker".map(|_| ProjectTypes::Docker).parse_next(s)
}

fn nodejs(s: &mut &str) -> PResult<ProjectTypes> {
    alt(("node", "js", "nodejs"))
        .map(|_| ProjectTypes::NodeJS)
        .parse_next(s)
}

fn rust(s: &mut &str) -> PResult<ProjectTypes> {
    "rust".map(|_| ProjectTypes::Rust).parse_next(s)
}

fn python(s: &mut &str) -> PResult<ProjectTypes> {
    "python".map(|_| ProjectTypes::Python).parse_next(s)
}

fn nix(s: &mut &str) -> PResult<ProjectTypes> {
    "nix".map(|_| ProjectTypes::Nix).parse_next(s)
}

fn project_type(s: &mut &str) -> PResult<ProjectTypes> {
    alt((git, docker, nodejs, rust, python, nix)).parse_next(s)
}

use thiserror::Error;
#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Unable to parse project type")]
    ProjectType,
}

impl FromStr for ProjectTypes {
    type Err = ParseError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        project_type.parse(s).map_err(|_| ParseError::ProjectType)
    }
}

impl Display for ProjectTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let repr = match self {
            ProjectTypes::Git => "git",
            ProjectTypes::Docker => "docker",
            ProjectTypes::NodeJS => "nodejs",
            ProjectTypes::Rust => "rust",
            ProjectTypes::Python => "python",
            ProjectTypes::Nix => "nix",
        };
        write!(f, "{}", repr)
    }
}

impl Default for ProjectTypes {
    fn default() -> Self {
        ProjectTypes::Git
    }
}

/// Determine direction to walk between root dir and cwd
pub enum TraversalDirection {
    /// Traverse from the root to the cwd
    Forward,
    /// Traverse from the cwd to root
    Backwards,
    /// Stay in cwd; do not traverse
    NoTraversal,
}

pub trait ProjectType {
    fn direction() -> TraversalDirection;
    fn condition(path: &Path) -> bool;
    fn traverse(path: &Path) -> Result<Option<PathBuf>> {
        match Self::direction() {
            TraversalDirection::Forward => traverse_forward(path, Self::condition),
            TraversalDirection::Backwards => traverse_backwards(path, Self::condition),
            TraversalDirection::NoTraversal => no_traversal(path, Self::condition),
        }
    }
}

pub struct RepoRoot<T: ProjectType> {
    pub path: PathBuf,
    pub project_type: PhantomData<T>,
}

impl<T> RepoRoot<T>
where
    T: ProjectType,
{
    pub fn new(path: &Path) -> Self {
        Self {
            path: path.to_path_buf(),
            project_type: PhantomData,
        }
    }

    /// Extract the path of the RepoRoot
    pub fn path(&self) -> PathBuf {
        self.path.clone()
    }

    pub fn find(path: &Path) -> Result<Option<Self>> {
        let root = T::traverse(path)?.map(|p| RepoRoot::new(&p));
        Ok(root)
    }
}
