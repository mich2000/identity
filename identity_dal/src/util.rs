use regex::Regex;

pub fn control_email(email: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new("...").unwrap();
    }
    RE.is_match(email)
}