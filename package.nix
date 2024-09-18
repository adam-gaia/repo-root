{inputs, pkgs, system, ...}:
let

  inherit (pkgs) lib;

  craneLib = inputs.crane.mkLib pkgs;
  src = craneLib.cleanCargoSource ./.;

  # Common arguments can be set here to avoid repeating them later
  commonArgs = {
    inherit src;
    strictDeps = true;

    buildInputs =
      [
        # Add additional build inputs here
      ]
      ++ lib.optionals pkgs.stdenv.isDarwin [
        # Additional darwin specific inputs can be set here
        pkgs.libiconv
      ];

    # Additional environment variables can be set directly
    # MY_CUSTOM_VAR = "some value";
  };

  craneLibLLvmTools =
    craneLib.overrideToolchain
    (inputs.fenix.packages.${system}.complete.withComponents [
      "cargo"
      "llvm-tools"
      "rustc"
    ]);

  # Build *just* the cargo dependencies, so we can reuse
  # all of that work (e.g. via cachix) when running in CI
  cargoArtifacts = craneLib.buildDepsOnly commonArgs;

  # Build the actual crate itself, reusing the dependency
  # artifacts from above.
  my-crate = craneLib.buildPackage (commonArgs
    // {
      inherit cargoArtifacts;
    });

  
  checks = {
    # Build the crate as part of `nix flake check` for convenience
    inherit my-crate;

    # Run clippy (and deny all warnings) on the crate source,
    # again, reusing the dependency artifacts from above.
    #
    # Note that this is done as a separate derivation so that
    # we can block the CI if there are issues here, but not
    # prevent downstream consumers from building our crate by itself.
    my-crate-clippy = craneLib.cargoClippy (commonArgs
      // {
        inherit cargoArtifacts;
        cargoClippyExtraArgs = "--all-targets -- --deny warnings";
      });

    my-crate-doc = craneLib.cargoDoc (commonArgs
      // {
        inherit cargoArtifacts;
      });

    # Check formatting
    my-crate-fmt = craneLib.cargoFmt {
      inherit src;
    };

    # Audit dependencies
    my-crate-audit = craneLib.cargoAudit {
      advisory-db = inputs.advisory-db;
      inherit src;
    };

    # Audit licenses
    my-crate-deny = craneLib.cargoDeny {
      inherit src;
    };

    # Run tests with cargo-nextest
    # Consider setting `doCheck = false` on `my-crate` if you do not want
    # the tests to run twice
    my-crate-nextest = craneLib.cargoNextest (commonArgs
      // {
        inherit cargoArtifacts;
        partitions = 1;
        partitionType = "count";
      });
    };
 
in
  my-crate // {
    passthru = my-crate.passthru // {
      tests = checks;
    };
  }
