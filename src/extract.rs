use std::collections::HashSet;
use std::error::Error;
use unrar::Archive;

pub const ZIP: &str = ".zip";
pub const RAR: &str = ".rar";
pub const SEVEN_Z: &str = ".7z";

pub fn extract_file(base: &str, file: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let mut archive = Archive::new(file).open_for_processing()?;
    let mut files = Vec::new();

    let files_to_extract = HashSet::from([
        "UserInformation.txt",
        "Passwords.txt",
        "All Passwords.txt",
    ]);

    while let Some(header) = archive.read_header()? {
        let entry = header.entry();

        archive = if entry.is_file() {
            match entry.filename.file_name().and_then(|os_str| os_str.to_str()) {
                Some(file_name) if files_to_extract.contains(&file_name.trim()) => {
                    files.push(entry.filename.to_string_lossy().to_string());
                    header.extract_with_base(base)?
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

#[cfg(test)]
mod tests {
    use crate::extract::extract_file;

    #[test]
    fn test() {
        let files = extract_file(
            "test_data/extract/",
            "test_data/@MANTICORECLOUD - 24.10 - 4300 PCS.rar",
        );

        assert_eq!(files.unwrap().len(), 159);
    }
}