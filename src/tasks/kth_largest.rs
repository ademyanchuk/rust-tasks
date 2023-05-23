use std::collections::BinaryHeap;

struct KthLargest {
    heap: BinaryHeap<i32>,
    k: usize,
}

/**
 * Your KthLargest object will be instantiated and called as such:
 * let obj = KthLargest::new(k, nums);
 * let ret_1: i32 = obj.add(val);
 */
impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut heap = BinaryHeap::new();
        let k: usize = k.try_into().expect("k must be k >= 1"); // constraints k >= 1
        for val in nums {
            heap.push(-val);
            if heap.len() > k {
                heap.pop();
            }
        }
        Self { heap, k }
    }

    fn add(&mut self, val: i32) -> i32 {
        self.heap.push(-val);
        if self.heap.len() > self.k {
            self.heap.pop();
        }
        -self.heap.peek().unwrap() // constraints k >= 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let mut klargest = KthLargest::new(1, vec![]);
        assert_eq!(klargest.add(1), 1)
    }
    #[test]
    fn test_add_only() {
        let mut klargest = KthLargest::new(2, vec![1]);
        assert_eq!(klargest.add(1), 1);
        assert_eq!(klargest.add(2), 1)
    }
}
