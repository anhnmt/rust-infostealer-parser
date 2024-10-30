use std::{
    collections::HashSet,
    error::Error,
};
use unrar::Archive;

pub const ZIP: &str = ".zip";
pub const RAR: &str = ".rar";
pub const SEVEN_Z: &str = ".7z";

#[derive(Debug, Default)]
pub struct Extract {
    pub base: &'static str,
    pub file: &'static str,
    pub whitelists: HashSet<&'static str>,
}

impl Extract {
    pub fn new(file: &'static str) -> Self {
        Self {
            file,
            ..Default::default()
        }
    }

    pub fn with_base(mut self, base: &'static str) -> Self {
        self.base = base;
        self
    }

    pub fn with_whitelists(mut self, whitelists: HashSet<&'static str>) -> Self {
        self.whitelists = whitelists;
        self
    }

    pub fn extract_file(self) -> Result<Vec<String>, Box<dyn Error>> {
        let mut archive = Archive::new(self.file).open_for_processing()?;
        let mut files: Vec<String> = Vec::new();

        while let Some(header) = archive.read_header()? {
            let entry = header.entry();

            archive = if entry.is_file() {
                match entry.filename.file_name().and_then(|os_str| os_str.to_str()) {
                    Some(file_name) if self.whitelists.is_empty() || self.whitelists.contains(&file_name.trim()) => {
                        files.push(entry.filename.to_string_lossy().to_string());
                        header.extract_with_base(self.base)?
                    }
                    _ => header.skip()?,
                }
            } else {
                header.skip()?
            };
        }

        // println!("{:?} files extracted", files);
        Ok(files)
    }
}

#[cfg(test)]
mod tests {
    use crate::extract::Extract;
    use std::{
        collections::HashSet,
        fs,
    };

    #[test]
    fn default() {
        let extract = Extract::new("test_data/@MANTICORECLOUD - 24.10.rar")
            .with_base("test_data/extract/")
            .extract_file()
            .unwrap();

        let _ = fs::remove_dir_all("test_data/extract");
        assert_eq!(extract.len(), 307);
    }

    #[test]
    fn whitelists() {
        let extract = Extract::new("test_data/@MANTICORECLOUD - 24.10.rar")
            .with_base("test_data/extract/")
            .with_whitelists(HashSet::from([
                "UserInformation.txt",
                "Passwords.txt",
                "All Passwords.txt",
            ]))
            .extract_file()
            .unwrap();

        let _ = fs::remove_dir_all("test_data/extract");
        assert_eq!(extract.len(), 38);
    }
}