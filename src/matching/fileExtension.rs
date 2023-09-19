#[derive(Debug, PartialEq)]
pub struct FileExtension(String);

impl FileExtension {
    pub fn from(extension: &str) -> FileExtension {
        if extension
            .chars()
            .all(|c| c.is_ascii_alphabetic() || c.is_ascii_digit())
        {
            return FileExtension(extension.to_owned());
        } else {
            panic!("must be only letters and numbers (ts/js/rs) and not (.ts/.js/.rs)")
        }
    }
}
