use crate::model::Credential;
use crate::util::{get_host_from_url, get_match_string};
use std::path::PathBuf;
use validator::Validate;

pub const URL: &str = "(URL|url):";
pub const USERNAME: &str = "(Username|username):";
pub const PASSWORD: &str = "(Password|password):";
pub const APPLICATION: &str = "(Application|application):";

pub fn parse(file_path: &str, body: &str) -> Option<Vec<Credential>> {
    let entries: Vec<&str> = body.split("===============").
        filter_map(|line| {
            Some(line.trim()).filter(|s| !s.is_empty() && !s.starts_with("*") && !s.starts_with("─"))
        }).
        collect();
    if entries.is_empty() {
        return None;
    }

    let mut credentials = Vec::new();

    for entry in entries {
        let lines: Vec<&str> = entry.lines().filter_map(|line| {
            Some(line.trim()).filter(|s| !s.is_empty())
        }).collect();

        let mut credential = Credential {
            output_dir: PathBuf::from(file_path).parent().unwrap().to_str().unwrap().to_string(),
            ..Default::default()
        };

        for line in lines {
            // URL
            if let Some(val) = get_match_string(URL, line) {
                if !val.is_empty() {
                    credential.host = get_host_from_url(val.trim());
                    credential.url = val;
                }
            }

            // USERNAME
            if let Some(val) = get_match_string(USERNAME, line) {
                if !val.is_empty() {
                    credential.username = val
                }
            }

            // PASSWORD
            if let Some(val) = get_match_string(PASSWORD, line) {
                if !val.is_empty() {
                    credential.password = val
                }
            }

            // APPLICATION
            if let Some(val) = get_match_string(APPLICATION, line) {
                if !val.is_empty() {
                    credential.application = val
                }
            }
        }

        if credential.validate().is_ok() {
            // println!("{:?}", credential);
            credentials.push(credential);
        }
    }

    Some(credentials)
}

#[cfg(test)]
mod tests {
    use crate::credential::meta::parse;
    use std::fs;

    #[test]
    fn meta_credentials() {
        let file_path = "./test_data/META_Passwords.txt";
        let body = fs::read_to_string(file_path).unwrap();
        let credentials = parse(file_path, body.as_str());
        assert_eq!(credentials.unwrap().len(), 3)
    }

    #[test]
    fn bradmax_credentials() {
        let file_path = "./test_data/BRADMAX_Passwords.txt";
        let body = fs::read_to_string(file_path).unwrap();
        let credentials = parse(file_path, body.as_str());
        assert_eq!(credentials.unwrap().len(), 2)
    }

    #[test]
    fn manticore_credentials() {
        let file_path = "./test_data/MANTICORE_Passwords.txt";
        let body = fs::read_to_string(file_path).unwrap();
        let credentials = parse(file_path, body.as_str());
        assert_eq!(credentials.unwrap().len(), 4)
    }

    #[test]
    fn redline_credentials() {
        let file_path = "./test_data/REDLINE_Passwords.txt";
        let body = fs::read_to_string(file_path).unwrap();
        let credentials = parse(file_path, body.as_str());
        assert_eq!(credentials.unwrap().len(), 1)
    }
}