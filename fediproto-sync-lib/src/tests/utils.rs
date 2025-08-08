use rstest::*;

use crate::utils::*;

/// Tests to ensure that `new_random_file_name()` returns 14 characters.
#[rstest]
fn new_random_file_name__is_expected_length() {
    let file_name = new_random_file_name(14, None);

    assert!(
        file_name.len() == 14,
        "'{}' is not 14 characters",
        file_name
    );
}

#[rstest]
#[case("mp4")]
#[case(".mp4")]
#[case("..mp4")]
fn new_random_file_name__mp4_file_extensions(#[case] file_extension: &str) {
    let file_name = new_random_file_name(14, Some(file_extension));

    println!("{} -> {}", file_extension, file_name);

    assert!(
        file_name.ends_with(".mp4") && !file_name.ends_with("..mp4"),
        "'{}' does not end with '.mp4'",
        file_name
    );
}
