/// Given an array of positive integers `arr` and a positive integer `k`,
/// finds the smallest sequence of continuous elements in `arr` that adds up to `k` or more.
/// Return the length of that smallest sequence. If no such sequence exists, return 0.
fn smallest_sequence(arr: &[u32], k: u32) -> usize {
    let mut ans = arr.len() + 1;
    let (mut start, mut end) = (0usize, 0usize);
    let mut run_sum = 0;
    while end < arr.len() {
        if run_sum < k {
            run_sum += arr[end];
            end += 1;
        } else {
            ans = ans.min(end - start);
            run_sum -= arr[start];
            start += 1;
        }
    }
    // finish contracting the window in case sum >= k
    while start < arr.len() && run_sum >= k {
        ans = ans.min(end - start);
        run_sum -= arr[start];
        start += 1;
    }
    if ans > arr.len() {
        return 0;
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero() {
        assert_eq!(smallest_sequence(&[1, 2, 3], 10), 0)
    }
    #[test]
    fn test_first() {
        assert_eq!(smallest_sequence(&[3, 2, 1], 3), 1)
    }
    #[test]
    fn test_last() {
        assert_eq!(smallest_sequence(&[1, 2, 3], 3), 1)
    }
    #[test]
    fn test_one_element() {
        assert_eq!(smallest_sequence(&[10], 9), 1)
    }
    #[test]
    fn test_complex() {
        assert_eq!(smallest_sequence(&[1, 2, 3, 1], 5), 2)
    }
    #[test]
    fn test_n_seq() {
        assert_eq!(smallest_sequence(&[1, 2, 3], 6), 3)
    }
}
