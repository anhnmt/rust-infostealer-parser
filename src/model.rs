use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate, Debug)]
pub struct Credential {
    pub host: String,
    pub url: String,
    pub username: String,
    pub password: String,
    pub application: String,
    pub output_dir: String,
}

#[derive(Serialize, Deserialize, Validate, Debug)]
pub struct UserInformation {
    pub ip_address: String,
    pub operating_system: String,
    pub file_location: String,
    pub country: String,
    pub location: String,
    pub log_date: String,
    pub user_name: String,
    pub hardware_id: String,
    pub machine_name: String,
    pub machine_id: String,
    pub output_dir: String,
}
