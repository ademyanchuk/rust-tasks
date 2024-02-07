use std::collections::HashMap;

/// Given a string s, sort it in decreasing order based on the frequency of the characters.
/// The frequency of a character is the number of times it appears in the string.
/// Return the sorted string. If there are multiple answers, return any of them.

pub fn frequency_sort(s: String) -> String {
    // build counter hashmap
    let ch_count: HashMap<char, usize> = s.chars().fold(HashMap::new(), |mut map, c| {
        *map.entry(c).or_insert(0) += 1;
        map
    });
    // convert to vec of tuples and sort
    let mut ch_count: Vec<_> = ch_count.into_iter().collect();
    ch_count.sort_by(|a, b| a.1.cmp(&b.1).reverse());
    // build a final string
    let mut res = String::new(); // Directly using String
    for (ch, freq) in ch_count {
        res.push_str(&ch.to_string().repeat(freq));
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mixed_characters() {
        let result = frequency_sort("tree".to_string());
        assert!(result == "eert" || result == "eetr");
    }

    #[test]
    fn test_same_frequency() {
        let result = frequency_sort("abacbc".to_string());
        assert!(
            result == "aabbcc"
                || result == "aaccbb"
                || result == "bbaacc"
                || result == "bbccaa"
                || result == "ccaabb"
                || result == "ccbbaa"
        );
    }

    #[test]
    fn test_single_character() {
        let result = frequency_sort("aaaa".to_string());
        assert_eq!(result, "aaaa");
    }

    #[test]
    fn test_empty_string() {
        let result = frequency_sort("".to_string());
        assert_eq!(result, "");
    }
}
