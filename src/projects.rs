use super::{ProjectType, TraversalDirection};
use std::path::Path;

pub struct NodeProject {}
impl ProjectType for NodeProject {
    fn direction() -> TraversalDirection {
        TraversalDirection::Backwards
    }
    fn condition(path: &Path) -> bool {
        let package = path.join("package.json");
        package.is_file()
    }
}

pub struct PythonProject {}
impl ProjectType for PythonProject {
    fn direction() -> TraversalDirection {
        TraversalDirection::Backwards
    }
    fn condition(path: &Path) -> bool {
        let toml = path.join("pyproject.toml");
        toml.is_file()
    }
}

pub struct RustProject {}
impl ProjectType for RustProject {
    fn direction() -> TraversalDirection {
        TraversalDirection::Backwards
    }
    fn condition(path: &Path) -> bool {
        let toml = path.join("Cargo.toml"); // TODO: handle workspace?
        toml.is_file()
    }
}

pub struct NixProject {}
impl ProjectType for NixProject {
    fn direction() -> TraversalDirection {
        TraversalDirection::Backwards
    }
    fn condition(path: &Path) -> bool {
        let flake = path.join("flake.nix"); // TODO: handle workspace?
        flake.is_file()
    }
}

pub struct GitProject {}
impl ProjectType for GitProject {
    fn direction() -> TraversalDirection {
        TraversalDirection::Backwards
    }
    fn condition(path: &Path) -> bool {
        let git_dir = path.join(".git");
        git_dir.is_dir()
    }
}

pub struct DockerProject {}
impl ProjectType for DockerProject {
    fn direction() -> TraversalDirection {
        TraversalDirection::Forward
    }
    fn condition(path: &Path) -> bool {
        let dockerfile = path.join("Dockerfile"); // TODO: ignore case, check for 'containerfile' too?
        dockerfile.is_file()
    }
}
