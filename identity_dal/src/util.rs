use regex::Regex;
use rand::{thread_rng, Rng};

pub fn control_email(email: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"[a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?").unwrap();
    }
    RE.is_match(email)
}

static HEXA_ALPHABET : [char;16] = ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0', 'a', 'b', 'c', 'd', 'e', 'f'];

/**
 * Returns a 8 character hash made off hexadecimal characters.
 */
pub fn get_hash(amount : usize) -> String {
    (0..amount).map(|_| HEXA_ALPHABET[thread_rng().gen_range(0, HEXA_ALPHABET.len())] as char ).collect()
}