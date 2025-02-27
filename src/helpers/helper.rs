pub fn convert_file_extension_to_raw(extension: &str) -> Option<Vec<u8>> {
    Some(extension.as_bytes().to_vec())
}
pub fn convert_raw_to_file_extension(raw: Vec<u8>) -> Option<String> {
    String::from_utf8(raw).ok()
}
