use crate::header::is_meta_group;
use crate::model::UserInformation;
use std::path::Path;

pub mod meta;

pub const WHITELISTS: [&str; 1] = ["UserInformation.txt"];

pub fn parse(file_path: &str, body: &str) -> Option<UserInformation> {
    let file_name = Path::new(file_path)
        .file_name()
        .and_then(|os_str| os_str.to_str())?;

    if WHITELISTS.contains(&file_name) {
        if is_meta_group(body) {
            return meta::parse(file_path, body);
        }
    }

    None
}
