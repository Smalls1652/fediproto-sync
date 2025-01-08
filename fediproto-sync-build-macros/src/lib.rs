pub extern crate git_version;
pub extern crate toml_edit;

/// Sets the package version based on the latest git tag and commit hash.
///
/// ## Environment Variables
///
/// * `FEDIPROTOSYNC_INCLUDE_COMMIT_HASH` - If set to `true`, the commit hash
///   will be included in the version string.
/// * `FEDIPROTOSYNC_UPDATE_MANIFEST_VERSION` - If set to `true`, the package
///   version in the manifest file will be updated.
#[macro_export]
macro_rules! set_package_version {
    () => {
        // Get the build profile and current package version.
        let build_profile_env_var = std::env::var("PROFILE").clone().unwrap();
        let package_version: &'static str = env!("CARGO_PKG_VERSION");
        let build_profile = build_profile_env_var.as_str();

        // Check if the commit hash should be included in the version string.
        let include_commit_hash = std::env::var("FEDIPROTOSYNC_INCLUDE_COMMIT_HASH")
            .unwrap_or("false".to_string())
            .parse::<bool>()
            .unwrap();

        // Determine the current git version based on the build profile and whether
        // the commit hash should be included.
        let current_git_version: String = match build_profile == "release" && !include_commit_hash {
            true => $crate::git_version::git_version!(
                args = ["--tags", "--abbrev=0"],
                fallback = package_version.to_string()
            )
            .trim_start_matches('v')
            .to_string(),

            false => {
                let git_version_tag_output = $crate::git_version::git_version!(
                    args = ["--tags", "--abbrev=0"],
                    fallback = package_version
                )
                .trim_start_matches('v');

                let git_version_output = $crate::git_version::git_version!(
                    args = ["--always"],
                    fallback = git_version_tag_output
                )
                .to_string();

                match &git_version_output == package_version {
                    true => package_version.to_string(),

                    false => format!("{}-{}", git_version_tag_output, git_version_output)
                }
            }
        };

        // Set the rustc environment variables for the current git version and package
        // version.
        println!("cargo::rustc-env=GIT_VERSION={}", current_git_version);
        println!("cargo::rustc-env=CARGO_PKG_VERSION={}", current_git_version);

        // Check if the package version should be updated in the manifest file.
        // The 'FEDIPROTOSYNC_UPDATE_MANIFEST_VERSION' environment variable should
        // be set to 'true' for this to happen.
        let should_update_cargo_manifest = std::env::var("FEDIPROTOSYNC_UPDATE_MANIFEST_VERSION")
            .unwrap_or("false".to_string())
            .parse::<bool>()
            .unwrap();

        // If the package version should be updated in the manifest file, update it.
        if should_update_cargo_manifest {
            println!(
                "cargo::warning=Package version will be set to '{}' in the manifest file.",
                current_git_version
            );

            let manifest_dir_path =
                std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());

            let cargo_toml_path = manifest_dir_path.join("Cargo.toml");

            let mut cargo_toml = std::fs::read_to_string(&cargo_toml_path)
                .expect("Failed to read Cargo.toml.")
                .parse::<$crate::toml_edit::DocumentMut>()
                .expect("Failed to parse Cargo.toml.");

            cargo_toml["package"]["version"] =
                $crate::toml_edit::value(&current_git_version.trim_start_matches("v").to_string());

            std::fs::write(&cargo_toml_path, cargo_toml.to_string())
                .expect("Failed to write Cargo.toml.");
        }
    };
}
