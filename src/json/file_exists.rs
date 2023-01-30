use std::path::PathBuf;

pub(crate) fn file_exists(file_path: PathBuf) -> bool {
    return file_path.exists();
}
