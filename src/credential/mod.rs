use crate::header::is_meta_group;
use crate::model::Credential;

pub mod meta;

pub const WHITELISTS: [&str; 2] = [
    "Passwords.txt",
    "All Passwords.txt"
];

pub fn parse(file_path: &str, body: &str) -> Option<Vec<Credential>> {
    if is_meta_group(body) {
        return meta::parse(file_path, body);
    }

    None
}