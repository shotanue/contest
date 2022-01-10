struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        for (i, &num) in nums.iter().enumerate() {
            let i = i as i32;
            if let Some(&ii) = map.get(&(target - num)) {
                return vec![ii, i];
            } else {
                map.insert(num, i);
            }
        }
        return vec![];
    }
}

#[cfg(test)]
mod test {
    use crate::problems::problems_1_two_sum::Solution;

    #[test]
    fn example_1() {
        assert_eq!(Solution::two_sum(vec![2, 7, 12, 15], 9), vec![0, 1]);
        assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
        assert_eq!(Solution::two_sum(vec![3, 3], 6), vec![0, 1]);
    }
}
