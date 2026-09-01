use std::path::Path;

const VALID_EXTENSIONS: [&str; 7] = ["png", "jpg", "jpeg", "gif", "bmp", "tiff", "webp"];

pub fn is_image_file(filepath: &Path) -> bool {
    filepath
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_lowercase())
        .is_some_and(|ext| VALID_EXTENSIONS.contains(&ext.as_str()))
}
