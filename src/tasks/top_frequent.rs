use std::collections::{BinaryHeap, HashMap};

pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut counter: HashMap<i32, i32> = HashMap::new();
    for val in nums {
        counter.entry(val).and_modify(|cnt| *cnt += 1).or_insert(1);
    }
    let mut heap = BinaryHeap::with_capacity(k as usize);
    for (i, (val, cnt)) in counter.iter().enumerate() {
        if i < k as usize {
            heap.push([-cnt, *val]);
        } else if let Some(kth) = heap.peek() {
            if -cnt < kth[0] {
                heap.pop();
                heap.push([-cnt, *val]);
            }
        }
    }
    heap.into_iter().map(|item| item[1]).collect::<Vec<i32>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        assert_eq!(top_k_frequent(vec![1], 1), vec![1]);
    }
    #[test]
    fn test_simple() {
        let mut ans = top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2);
        ans.sort();
        assert_eq!(ans, vec![1, 2])
    }
}
