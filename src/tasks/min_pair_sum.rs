// The pair sum of a pair (a,b) is equal to a + b. The maximum pair sum is the largest pair sum in a list of pairs.
// For example, if we have pairs (1,5), (2,3), and (4,4), the maximum pair sum would be max(1+5, 2+3, 4+4) = max(6, 5, 8) = 8.
// Given an array nums of even length n, pair up the elements of nums into n / 2 pairs such that:
// Each element of nums is in exactly one pair, and
// The maximum pair sum is minimized.
// Return the minimized maximum pair sum after optimally pairing up the elements.
pub fn min_pair_sum(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    nums.sort_unstable();
    let mut answer = i32::MIN;
    let n = nums.len() / 2;
    let (mut left, mut right) = (0usize, nums.len() - 1);
    while left < n && right >= n {
        answer = answer.max(nums[left] + nums[right]);
        left += 1;
        right -= 1;
    }
    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evenly_distributed_elements() {
        assert_eq!(min_pair_sum(vec![1, 2, 3, 4]), 5);
    }

    #[test]
    fn test_large_range_of_elements() {
        assert_eq!(min_pair_sum(vec![10, 20, 30, 40, 50, 60]), 70);
    }

    #[test]
    fn test_duplicate_elements() {
        assert_eq!(min_pair_sum(vec![5, 5, 5, 5]), 10);
    }

    #[test]
    fn test_single_pair() {
        assert_eq!(min_pair_sum(vec![100, 1]), 101);
    }

    #[test]
    fn test_negative_and_positive_numbers() {
        assert_eq!(min_pair_sum(vec![-1, -2, 3, 4]), 2);
    }

    #[test]
    fn test_zero_elements() {
        assert_eq!(min_pair_sum(vec![0, 0, 0, 0]), 0);
    }
}
