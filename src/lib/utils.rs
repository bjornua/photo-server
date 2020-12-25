const MATCH_EXTENSIONS: &'static [&'static str] = &["jpg", "JPG", "png", "PNG"];

fn str_matches_ext(a: &std::ffi::OsStr) -> bool {
    MATCH_EXTENSIONS
        .iter()
        .any(|b| a == std::ffi::OsStr::new(b))
}

pub fn path_matches_ext<P: AsRef<std::path::Path>>(path: &P) -> bool {
    path.as_ref()
        .extension()
        .map(str_matches_ext)
        .unwrap_or(false)
}
