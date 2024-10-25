use regex::Regex;
use std::error::Error;

const META_HEADER: &str = r#"
\*               _   _   _   _                 \*
\*              / \\ / \\ / \\ / \\                \*
\*             \( M \| E \| T \| A \)               \*
\*              \\_/ \\_/ \\_/ \\_/                \*
"#;

pub fn is_match_header(header: &str, body: &str) -> Result<bool, Box<dyn Error>> {
    if body.trim().is_empty() {
        return Ok(false);
    }

    let body_segment: String = body.lines().
        take(50).
        filter_map(|line| {
            Some(line.trim()).filter(|s| !s.is_empty())
        }).
        collect::<Vec<&str>>().
        join("\n");

    for line in header.lines().filter_map(|line| {
        Some(line.trim()).filter(|s| !s.is_empty())
    }) {
        if !Regex::new(line)?.is_match(&body_segment) {
            return Ok(false);
        }
    }

    Ok(true)
}

#[cfg(test)]
mod tests {
    use crate::header::*;
    use std::fs;

    #[test]
    fn meta_userinfo() {
        let message = fs::read_to_string("./test_data/META_UserInformation.txt").unwrap();
        let check = is_match_header(META_HEADER, message.as_str()).unwrap();

        assert_eq!(check, true);
    }

    #[test]
    fn meta_password() {
        let message = fs::read_to_string("./test_data/META_Passwords.txt").unwrap();
        let check = is_match_header(META_HEADER, message.as_str()).unwrap();

        assert_eq!(check, true);
    }
}