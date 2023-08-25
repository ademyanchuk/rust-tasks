/// Given a String which represents a valid sentence. A sentence is considered valid if it follows the following rules:
///
/// - It starts with a capital letter.
/// - It ends with a full stop (.) character.
/// - Between the first letter and the last full stop, there may only be lowercase letters and space characters.
fn is_valid_sentence(s: &str) -> bool {
    let mut chars: Vec<char> = s.chars().collect();
    if !chars[0].is_uppercase() || chars.pop().unwrap() != '.' {
        return false;
    }
    for ch in chars.iter().skip(1) {
        if !(ch.is_lowercase() || ch.is_whitespace()) {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        assert!(!is_valid_sentence("A"))
    }
    #[test]
    fn test_two_valid() {
        assert!(is_valid_sentence("A."))
    }
    #[test]
    fn test_simple_valid() {
        assert!(is_valid_sentence("Hello world."))
    }
    #[test]
    fn test_numbers() {
        assert!(!is_valid_sentence("Hello wo45d."))
    }
    #[test]
    fn test_special() {
        assert!(!is_valid_sentence("Hello/ \\world."))
    }
}
