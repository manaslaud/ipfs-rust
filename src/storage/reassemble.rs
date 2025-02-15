use infer::get;

pub fn detect_file_type(file_data: &[u8]) -> String {
   let kind= get(file_data).expect("Unknown file type");
   kind.mime_type().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_detect_file_type() {
        let data: Vec<u8> = vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
        let kind = detect_file_type(&data);
        assert_eq!(kind, "image/png");
    }
}