use crate::header::is_meta_group;
use crate::model::UserInformation;

pub mod meta;

pub const WHITELISTS: [&str; 1] = [
    "UserInformation.txt"
];

pub fn parse(file_path: &str, body: &str) -> Option<UserInformation> {
    if is_meta_group(body) {
        return meta::parse(file_path, body);
    }

    None
}