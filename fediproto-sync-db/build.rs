use fediproto_sync_build_macros::set_package_version;

fn main() {
    println!("cargo:rerun-if-changed=./migrations/");

    set_package_version!();
}
