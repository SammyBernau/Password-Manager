use std::path::{Path, PathBuf};

pub(crate) fn file_exists(file_path: PathBuf) -> bool {
    return file_path.exists();
}