use std::{env, fs, path::PathBuf};

use git_version::git_version;
use toml_edit::{value, DocumentMut};

const CARGO_PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Sets the version of the package to the latest git tag.
fn set_version() {
    let build_profile_env_var = env::var("PROFILE").clone().unwrap();

    let build_profile = build_profile_env_var.as_str();

    let current_git_version: String = match build_profile {
        "release" => git_version!(args = ["--tags", "--abbrev=0"], fallback = CARGO_PKG_VERSION.to_string()).to_string(),

        _ => {
            let git_version_tag_output = git_version!(args = ["--tags"], fallback = CARGO_PKG_VERSION).trim_start_matches('v');

            let git_version_output = git_version!(args = ["--always"], fallback = git_version_tag_output).to_string();

            match &git_version_output == CARGO_PKG_VERSION {
                true => CARGO_PKG_VERSION.to_string(),

                false => format!("{}-{}", git_version_tag_output, git_version_output)
            }
        }
    };

    println!("cargo::rustc-env=GIT_VERSION={}", current_git_version);
    println!("cargo::rustc-env=CARGO_PKG_VERSION={}", current_git_version);

    // Check if the package version should be updated in the manifest file.
    // The 'FEDIPROTOSYNC_UPDATE_MANIFEST_VERSION' environment variable should
    // be set to 'true' for this to happen.
    let should_update_cargo_manifest = env::var("FEDIPROTOSYNC_UPDATE_MANIFEST_VERSION")
        .unwrap_or("false".to_string())
        .parse::<bool>()
        .unwrap();

    if should_update_cargo_manifest {
        println!(
            "cargo::warning=Package version will be set to '{}' in the manifest file.",
            current_git_version
        );

        let manifest_dir_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

        let cargo_toml_path = manifest_dir_path.join("Cargo.toml");

        let mut cargo_toml = fs::read_to_string(&cargo_toml_path)
            .expect("Failed to read Cargo.toml.")
            .parse::<DocumentMut>()
            .expect("Failed to parse Cargo.toml.");

        cargo_toml["package"]["version"] =
            value(&current_git_version.trim_start_matches("v").to_string());

        fs::write(&cargo_toml_path, cargo_toml.to_string()).expect("Failed to write Cargo.toml.");
    }
}

fn main() {
    println!("cargo:rerun-if-changed=./migrations/");

    set_version();
}
