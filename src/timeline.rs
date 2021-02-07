use crate::lib::iterdir;

pub fn get_pictures() -> Vec<std::path::PathBuf> {
    let files: Result<Vec<_>, _> =
        iterdir::findfiles_with_ext(std::ffi::OsString::from("./pictures")).collect();
    files.unwrap()
}
