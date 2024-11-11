use regex::Regex;
use std::str::FromStr;
use url::Url;

pub fn get_match_string(pattern: &str, line: &str) -> Option<String> {
    let re = Regex::new(pattern).unwrap();
    if re.is_match(line) {
        return Some(re.replace_all(line, "").trim().to_string());
    }

    None
}

pub fn get_host_from_url(raw: &str) -> String {
    if let Ok(url) = Url::from_str(raw) {
        if let Some(host) = url.host_str() {
            if url.scheme() == "android" {
                return host.split('.').rev().collect::<Vec<_>>().join(".");
            }
            return host.to_string();
        }
    }

    raw.to_string()
}
