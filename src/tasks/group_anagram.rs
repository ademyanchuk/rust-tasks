use std::collections::HashMap;

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut anagrams: HashMap<String, Vec<String>> = HashMap::new();
    for s in strs.into_iter() {
        let mut k: Vec<char> = s.chars().collect();
        k.sort_unstable();
        let k: String = k.into_iter().collect();
        anagrams.entry(k).or_default().push(s);
    }
    anagrams.into_values().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let strs = vec![
            "eat".into(),
            "tea".into(),
            "tan".into(),
            "ate".into(),
            "nat".into(),
            "bat".into(),
        ];
        let mut result = group_anagrams(strs);
        // Sort each inner vec and the outer vec for comparison purposes
        for group in &mut result {
            group.sort();
        }
        result.sort();
        assert_eq!(
            result,
            vec![vec!["ate", "eat", "tea"], vec!["bat"], vec!["nat", "tan"]]
        );
    }

    #[test]
    fn test_empty_string() {
        let strs = vec!["".into(), "".into(), "a".into()];
        let mut result = group_anagrams(strs);
        for group in &mut result {
            group.sort();
        }
        result.sort();
        assert_eq!(result, vec![vec!["", ""], vec!["a"]]);
    }

    #[test]
    fn test_single_character_strings() {
        let strs = vec!["a".into(), "b".into(), "c".into(), "a".into()];
        let mut result = group_anagrams(strs);
        for group in &mut result {
            group.sort();
        }
        result.sort();
        assert_eq!(result, vec![vec!["a", "a"], vec!["b"], vec!["c"]]);
    }

    #[test]
    fn test_no_anagrams() {
        let strs = vec!["abc".into(), "def".into(), "ghi".into()];
        let mut result = group_anagrams(strs);
        for group in &mut result {
            group.sort();
        }
        result.sort();
        // Expect each string in its own group since there are no anagrams
        assert_eq!(result, vec![vec!["abc"], vec!["def"], vec!["ghi"]]);
    }
}
