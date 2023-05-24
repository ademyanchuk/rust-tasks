use std::collections::BinaryHeap;

// 2542. Maximum Subsequence Score
pub fn max_score(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i64 {
    let k = k as usize;
    let mut pairs: Vec<(i32, i32)> = nums1.into_iter().zip(nums2).collect();
    pairs.sort_by_key(|pair| -pair.1);
    // we want min heap here
    let mut k_heap = BinaryHeap::from_iter(pairs.iter().take(k).map(|pair| -pair.0));
    let mut run_sum: i64 = k_heap.iter().map(|val| -val as i64).sum();
    let mut score = run_sum * pairs[k - 1].1 as i64;
    for (val, cur_min) in pairs.into_iter().skip(k) {
        k_heap.push(-val);
        run_sum += val as i64;
        run_sum += k_heap.pop().unwrap() as i64; // += because of inverted sign
        score = score.max(run_sum * cur_min as i64);
    }
    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        assert_eq!(max_score(vec![1], vec![1], 1), 1);
    }
    #[test]
    fn test_simple() {
        assert_eq!(max_score(vec![1, 3, 3, 2], vec![2, 1, 3, 4], 3), 12)
    }
    #[test]
    fn test_simple_2() {
        assert_eq!(max_score(vec![2, 1, 14, 12], vec![11, 7, 13, 6], 3), 168)
    }
}
