
struct Solution{}

/// https://leetcode.com/explore/learn/card/fun-with-arrays/521/introduction/3240/
impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        use std::collections::BinaryHeap;
        let mut heap = BinaryHeap::new();
        for num in nums {
            heap.push(num * num);
        }
        return heap.into_sorted_vec();
    }
}