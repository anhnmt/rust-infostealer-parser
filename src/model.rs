use std::hash::{Hash, Hasher};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Default, Serialize, Deserialize, Validate, Debug, Eq)]
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

impl Credential {
    fn sum(&self) -> String {
        format!("{}|{}|{}|{}", self.url, self.username, self.password, self.application)
    }
}

impl Hash for Credential {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.sum().hash(state);
    }
}

impl PartialEq for Credential {
    fn eq(&self, other: &Self) -> bool {
        self.sum().eq(&other.sum())
    }
}

#[derive(Default, Serialize, Deserialize, Validate, Debug, Eq)]
pub struct UserInformation {
    pub ip_address: String,
    pub operating_system: String,
    pub file_location: String,
    pub country: String,
    pub location: String,
    pub log_date: DateTime<Utc>,
    pub user_name: String,
    #[validate(length(min = 1))]
    pub hardware_id: String,
    pub machine_name: String,
    pub machine_id: String,
    pub output_dir: String,
}

impl UserInformation {
    fn sum(&self) -> String {
        format!("{}|{}|{}|{}|{}", self.ip_address, self.user_name, self.machine_name, self.country, self.hardware_id)
    }
}

impl Hash for UserInformation {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.sum().hash(state)
    }
}

impl PartialEq for UserInformation {
    fn eq(&self, other: &Self) -> bool {
        self.sum().eq(&other.sum())
    }
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
            hardware_id: "ED91HGTYFOXM9OCK7444W8T641IZQ3AB".to_string(),
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