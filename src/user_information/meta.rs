use crate::model::UserInformation;
use crate::util::get_match_string;
use chrono::{DateTime, NaiveDateTime, Utc};
use std::path::PathBuf;
use validator::Validate;

pub const IP_ADDRESS: &str = "(IP|ip):";
pub const FILE_LOCATION: &str = "(FileLocation):";
pub const USER_NAME: &str = "(UserName):";
pub const MACHINE_NAME: &str = "(MachineName):";
pub const MACHINE_ID: &str = "(MachineID):";
pub const COUNTRY: &str = "(Country):";
pub const LOCATION: &str = "^(Location):";
pub const HARDWARE_ID: &str = "(HWID):";
pub const OPERATING_SYSTEM: &str = "(Operation System):";
pub const LOG_DATE: &str = "(Log date):";

pub fn parse(file_path: &str, body: &str) -> Option<UserInformation> {
    let lines: Vec<&str> = body.lines()
        .filter_map(|line| {
            Some(line.trim()).filter(|s| !s.is_empty() && !s.starts_with("*") && !s.starts_with("─"))
        })
        .collect();
    if lines.is_empty() {
        return None;
    }

    let mut user_information = UserInformation {
        output_dir: PathBuf::from(file_path).parent().unwrap().to_str().unwrap().to_string(),
        ..Default::default()
    };

    for line in lines {
        // IP_ADDRESS
        if let Some(val) = get_match_string(IP_ADDRESS, line) {
            if !val.is_empty() {
                user_information.ip_address = val;
            }
        }

        // FILE_LOCATION
        if let Some(val) = get_match_string(FILE_LOCATION, line) {
            if !val.is_empty() {
                user_information.file_location = val;
            }
        }

        // USER_NAME
        if let Some(val) = get_match_string(USER_NAME, line) {
            if !val.is_empty() {
                user_information.user_name = val;
            }
        }

        // MACHINE_NAME
        if let Some(val) = get_match_string(MACHINE_NAME, line) {
            if !val.is_empty() {
                user_information.machine_name = val;
            }
        }

        // MACHINE_ID
        if let Some(val) = get_match_string(MACHINE_ID, line) {
            if !val.is_empty() {
                user_information.machine_id = val;
            }
        }

        // COUNTRY
        if let Some(val) = get_match_string(COUNTRY, line) {
            if !val.is_empty() {
                user_information.country = val;
            }
        }

        // LOCATION
        if let Some(val) = get_match_string(LOCATION, line) {
            if !val.is_empty() {
                user_information.location = val;
            }
        }

        // HARDWARE_ID
        if let Some(val) = get_match_string(HARDWARE_ID, line) {
            if !val.is_empty() {
                user_information.hardware_id = val;
            }
        }

        // OPERATING_SYSTEM
        if let Some(val) = get_match_string(OPERATING_SYSTEM, line) {
            if !val.is_empty() {
                user_information.operating_system = val;
            }
        }

        // // LOG_DATE
        if let Some(val) = get_match_string(LOG_DATE, line) {
            if !val.is_empty() {
                if let Some(log_date) = parse_log_date(val.as_str()) {
                    user_information.log_date = log_date;
                }
            }
        }
    }

    if user_information.validate().is_err() {
        return None;
    }

    // println!("{:?}\n", user_information);
    Some(user_information)
}

fn parse_log_date(log_date: &str) -> Option<DateTime<Utc>> {
    let formats = [
        "%m/%d/%Y %H:%M:%S",
        "%-m/%d/%Y %H:%M:%S",
        "%-m/%d/%Y %H:%M:%S %p",
        "%-m/%-d/%Y %H:%M:%S %p",
        "%a %b %e %H:%M:%S %Y", // equivalent to time.ANSIC in Go
    ];

    for format in &formats {
        if let Ok(naive_datetime) = NaiveDateTime::parse_from_str(log_date, format) {
            return Some(naive_datetime.and_utc());
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use crate::user_information::meta::parse;
    use std::fs;

    #[test]
    fn meta_user_information() {
        let file_path = "./test_data/META_UserInformation.txt";
        let body = fs::read_to_string(file_path).unwrap();
        let user_information = parse(file_path, body.as_str());
        assert_eq!(user_information.is_some(), true)
    }

    #[test]
    fn bradmax_user_information() {
        let file_path = "./test_data/BRADMAX_UserInformation.txt";
        let body = fs::read_to_string(file_path).unwrap();
        let user_information = parse(file_path, body.as_str());
        assert_eq!(user_information.is_some(), true)
    }

    #[test]
    fn manticore_user_information() {
        let file_path = "./test_data/MANTICORE_UserInformation.txt";
        let body = fs::read_to_string(file_path).unwrap();
        let user_information = parse(file_path, body.as_str());
        assert_eq!(user_information.is_some(), true)
    }

    #[test]
    fn redline_user_information() {
        let file_path = "./test_data/REDLINE_UserInformation.txt";
        let body = fs::read_to_string(file_path).unwrap();
        let user_information = parse(file_path, body.as_str());
        assert_eq!(user_information.is_some(), true)
    }
}