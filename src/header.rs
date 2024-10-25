use regex::{Error, Regex};

const META_HEADER: &str = r#"
\*               _   _   _   _                 \*
\*              / \\ / \\ / \\ / \\                \*
\*             \( M \| E \| T \| A \)               \*
\*              \\_/ \\_/ \\_/ \\_/                \*
"#;

fn is_match_header(header: &str, body: &str) -> Result<bool, Error> {
    for line in header.lines().map(str::trim).filter(|&line| !line.is_empty()) {
        if !Regex::new(line)?.is_match(body) {
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
    fn meta_header() {
        let message = fs::read_to_string("./test_data/META_UserInformation.txt").unwrap();
        let check = is_match_header(META_HEADER, message.as_str()).unwrap();

        assert_eq!(check, true);
    }
}