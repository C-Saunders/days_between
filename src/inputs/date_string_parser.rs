use regex::{Captures, Regex};
use DateFormat;

pub struct ParsedDateString {
    pub year: i32,
    pub month: u32,
    pub day: u32,
    pub format_type: DateFormat,
}

impl ParsedDateString {
    pub fn new(input: &str) -> Result<ParsedDateString, &'static str> {
        let re = Regex::new(r"^(?P<year>\d{4})-?(?P<month>\d{2})-?(?P<day>\d{2})$").unwrap();
        let captures = re.captures(input);

        match captures {
            Some(caps) => Ok(ParsedDateString {
                year: extract_as_int(&caps, &"year"),
                month: extract_as_unsigned(&caps, &"month"),
                day: extract_as_unsigned(&caps, &"day"),
                format_type: if input.contains("-") {
                    DateFormat::Dashes
                } else {
                    DateFormat::NoDashes
                },
            }),
            _ => Err("Failed to parse date string"),
        }
    }
}

// There's probably a nice way to do this with one function
// but I don't know it and it makes me a little sad
fn extract_as_int(caps: &Captures, group_name: &str) -> i32 {
    caps.name(group_name).unwrap().as_str().parse().unwrap()
}

fn extract_as_unsigned(caps: &Captures, group_name: &str) -> u32 {
    caps.name(group_name).unwrap().as_str().parse().unwrap()
}

#[cfg(test)]
mod tests {
    use super::ParsedDateString;

    #[test]
    fn parse_no_dashes() {
        let val: ParsedDateString = ParsedDateString::new("20180124").unwrap();
        assert_eq!(val.year, 2018);
        assert_eq!(val.month, 1);
        assert_eq!(val.day, 24);
    }

    #[test]
    fn parse_with_dashes() {
        let val: ParsedDateString = ParsedDateString::new("2018-01-24").unwrap();
        assert_eq!(val.year, 2018);
        assert_eq!(val.month, 1);
        assert_eq!(val.day, 24);
    }

    #[test]
    fn parse_fail_no_dashes() {
        assert!(ParsedDateString::new("2018124").is_err());
    }

    #[test]
    fn parse_fail_with_dashes() {
        assert!(ParsedDateString::new("2018-01-2").is_err());
    }
}
