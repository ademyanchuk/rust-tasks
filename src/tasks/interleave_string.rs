use std::collections::HashMap;

pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
    let b1 = s1.as_bytes();
    let b2 = s2.as_bytes();
    let b3 = s3.as_bytes();
    let mut memo = HashMap::new();
    recursive(b1, b2, b3, 0, 0, 0, &mut memo)
}

fn recursive(
    b1: &[u8],
    b2: &[u8],
    b3: &[u8],
    ix1: usize,
    ix2: usize,
    ix3: usize,
    memo: &mut HashMap<(usize, usize, usize), bool>,
) -> bool {
    if ix1 == b1.len() && ix2 == b2.len() && ix3 == b3.len() {
        return true;
    }
    if (b1.len() - ix1) + (b2.len() - ix2) != b3.len() - ix3 {
        return false;
    }
    if let Some(&res) = memo.get(&(ix1, ix2, ix3)) {
        return res;
    }
    let mut left = false;
    let mut right = false;
    if ix1 < b1.len() && b1[ix1] == b3[ix3] {
        left = recursive(b1, b2, b3, ix1 + 1, ix2, ix3 + 1, memo);
    }
    if ix2 < b2.len() && b2[ix2] == b3[ix3] {
        right = recursive(b1, b2, b3, ix1, ix2 + 1, ix3 + 1, memo);
    }
    let res = left || right;
    if res {
        memo.insert((ix1, ix2, ix3), res);
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        assert!(is_interleave(
            "".to_string(),
            "".to_string(),
            "".to_string()
        ))
    }
    #[test]
    fn test_false() {
        assert!(!is_interleave(
            "aabcc".to_string(),
            "dbbca".to_string(),
            "aadbbbaccc".to_string()
        ))
    }
    #[test]
    fn test_true() {
        assert!(is_interleave(
            "aabcc".to_string(),
            "dbbca".to_string(),
            "aadbbcbcac".to_string()
        ))
    }
}
