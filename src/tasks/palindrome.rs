use std::collections::HashMap;

use unicode_segmentation::UnicodeSegmentation;

/// Takes a vector of strings and returns a vector of pairs of indices (as vectors of two integers)
/// where the concatenated strings form a palindrome
pub fn palindrome_pairs(words: Vec<String>) -> Vec<Vec<i32>> {
    let max_cap = words.len().pow(2);
    let mut answer: Vec<Vec<i32>> = Vec::with_capacity(max_cap);
    // using hash map makes average performance to be linear
    // only the input of all same words like [cat, cat, cat] gives quadratic time
    let mut mem: HashMap<String, Vec<usize>> = HashMap::with_capacity(words.len());
    for (i, word) in words.iter().enumerate() {
        if let Some(indices) = mem.get(word) {
            for j in indices {
                answer.push(vec![*j as i32, i as i32]);
                answer.push(vec![i as i32, *j as i32]);
            }
        }
        // grapheme helps to deal with unicode reversing
        let rev_word = words[i].graphemes(true).rev().collect::<String>();
        mem.entry(rev_word).or_insert_with(Vec::new).push(i);
    }
    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_palindrome_pairs() {
        let mut ans = palindrome_pairs(vec![
            "bat".to_string(),
            "tab".to_string(),
            "cat".to_string(),
        ]);
        ans.sort();
        assert_eq!(ans, vec![vec![0, 1], vec![1, 0]]);
        let mut ans = palindrome_pairs(vec!["".to_string(), "".to_string(), "cat".to_string()]);
        ans.sort();
        assert_eq!(ans, vec![vec![0, 1], vec![1, 0]]);
        let mut ans = palindrome_pairs(vec![
            "bat".to_string(),
            "tab".to_string(),
            "cat".to_string(),
            "bat".to_string(),
        ]);
        ans.sort();
        assert_eq!(ans, vec![vec![0, 1], vec![1, 0], vec![1, 3], vec![3, 1]])
        // Add more test cases here
    }
}
