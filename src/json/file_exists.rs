use std::path::Path;



pub(crate) fn file_exists(file_path: &str) -> bool {
    let from_string = Path::new(&file_path);

    let existence = Path::new(from_string).exists();

    if existence == true {
        true
    } else {
        false
    }
}