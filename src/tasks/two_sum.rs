use std::collections::HashMap;

fn two_sum(nums: &[i32], target: i32) -> Option<(usize, usize)> {
    let mut map = HashMap::with_capacity(nums.len());
    for (i, val) in nums.iter().enumerate() {
        if let Some(j) = map.get(val) {
            return Some((i, *j));
        }
        map.insert(target - val, i);
    }
    None
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_or_one() {
        assert_eq!(two_sum(&[1], 1), None);
        assert_eq!(two_sum(&[], 1), None)
    }
    #[test]
    fn test_simple() {
        assert_eq!(two_sum(&[1, 8, 4, 20, 5], 28), Some((3, 1)))
    }
    #[test]
    fn test_negative() {
        assert_eq!(two_sum(&[-2, 1, 2], 0), Some((2, 0)))
    }
    #[test]
    fn test_duplicate() {
        assert_eq!(two_sum(&[0, 10, 10, 0], 0), Some((3, 0)))
    }
}
