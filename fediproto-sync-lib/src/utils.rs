use rand::distr::SampleString;

/// Generate a new random file name.
/// 
/// ## Arguments
/// 
/// * `length` - The length of the random file name.
/// * `file_extension` - An optional file extension to append. The leading `.` is not needed.
pub fn new_random_file_name(
    length: usize,
    file_extension: Option<&str>
) -> String {
    let rng = &mut rand::rng();

    let random_string = rand::distr::Alphanumeric.sample_string(rng, length);

    return match file_extension {
        Some(ext) => format!(
            "{}.{}",
            random_string,
            ext.trim_start_matches('.')
        ),

        None => random_string
    }
}
