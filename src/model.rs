use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Default, Serialize, Deserialize, Validate, Debug)]
pub struct Credential {
    pub host: String,
    #[validate(length(min = 1))]
    pub url: String,
    #[validate(length(min = 1))]
    pub username: String,
    #[validate(length(min = 1))]
    pub password: String,
    pub application: String,
    pub output_dir: String,
}

#[derive(Default, Serialize, Deserialize, Validate, Debug)]
pub struct UserInformation {
    #[validate(length(min = 1))]
    pub ip_address: String,
    pub operating_system: String,
    pub file_location: String,
    pub country: String,
    pub location: String,
    pub log_date: DateTime<Utc>,
    pub user_name: String,
    pub hardware_id: String,
    pub machine_name: String,
    pub machine_id: String,
    pub output_dir: String,
}

#[cfg(test)]
mod tests {
    use crate::model::*;
    use validator::Validate;

    #[test]
    fn credential_ok() {
        let credential = Credential {
            url: "https://www.facebook.com/login.php".to_string(),
            username: "32138160511".to_string(),
            password: "43500461Mk".to_string(),
            ..Default::default()
        };

        assert_eq!(credential.validate().is_ok(), true)
    }

    #[test]
    fn credential_err() {
        let credential = Credential {
            ..Default::default()
        };

        assert_eq!(credential.validate().is_err(), true)
    }

    #[test]
    fn user_info_ok() {
        let user_info = UserInformation {
            ip_address: "https://www.facebook.com/login.php".to_string(),
            ..Default::default()
        };

        assert_eq!(user_info.validate().is_ok(), true)
    }

    #[test]
    fn user_info_err() {
        let user_info = UserInformation {
            ..Default::default()
        };

        assert_eq!(user_info.validate().is_err(), true)
    }
}