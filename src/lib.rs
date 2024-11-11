use crate::model::{Credential, UserInformation};
use extract::Extract;
use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::path::Path;

pub mod credential;
pub mod extract;
pub mod header;
pub mod model;
pub mod user_information;
pub mod util;

#[derive(Default)]
pub struct Parser {
    pub user_information: Vec<UserInformation>,
    pub credentials: Vec<Credential>,
}

pub fn parse(base: &str, file: &str) -> Result<Parser, Box<dyn Error>> {
    let whitelists: HashSet<&str> = user_information::WHITELISTS
        .iter()
        .chain(credential::WHITELISTS.iter())
        .copied()
        .collect();

    let extract = Extract::new(file)
        .with_base(base)
        .with_whitelists(whitelists)
        .extract_file()?;

    let mut parser = Parser::default();

    for filename in extract {
        let trimmed_file = filename.trim();
        let file_path = Path::new(base).join(trimmed_file);

        if let Ok(body) = fs::read_to_string(file_path) {
            // Find user information first
            match user_information::parse(trimmed_file, &body) {
                Some(user_info) => {
                    parser.user_information.push(user_info);
                }
                // If user information not found, try credentials
                None => {
                    if let Some(credentials) = credential::parse(trimmed_file, &body) {
                        parser.credentials.extend(credentials);
                    }
                }
            }
        }
    }

    Ok(parser)
}

#[cfg(test)]
mod tests {
    // use crate::parse;
    // use std::fs;

    // #[test]
    // fn manticore_parser() {
    //     let base = "test_data/@MANTICORECLOUD - 24.10";
    //     let file = "test_data/@MANTICORECLOUD - 24.10.rar";
    //     let parser = parse(base, file).unwrap();
    //     let _ = fs::remove_dir_all(base);
    //     assert_eq!(parser.user_information.len(), 15);
    //     assert_eq!(parser.credentials.len(), 386)
    // }

    // #[test]
    // fn godelesscloud_parser() {
    //     let base = "test_data/GODELESS CLOUD";
    //     let file = "test_data/GODELESS CLOUD.rar";
    //     let parser = parse(base, file).unwrap();
    //     let _ = fs::remove_dir_all(base);
    //     assert_eq!(parser.user_information.len(), 20);
    //     assert_eq!(parser.credentials.len(), 520)
    // }
}