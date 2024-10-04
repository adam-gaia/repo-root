<div class="oranda-hide">

# repo-root

</div>

Find the root directory of a project.

## CLI

```bash
    cargo install repo-root
```

## Library

```bash
  cargo add repo-root
```

## Supported project types

- git:`.git` dir
- Docker: `Dockerfile` file
- NodeJS: `package.json` file
- Rust: `Cargo.toml` file
- Python: `pyproject.toml` file
- Nix: `flake.nix` file

### Custom

See [examples/custom_project_type.rs](examples/custom_project_type.rs)
