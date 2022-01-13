struct Solution {}

impl Solution {
    // https://leetcode.com/explore/learn/card/fun-with-arrays/521/introduction/3237/
    pub fn find_numbers(nums: Vec<i32>) -> i32 {
        let mut acc = 0;
        for num in nums {
            match num {
                x if 10 <= x && x < 100 => acc = acc + 1,
                x if 1000 <= x && x < 10_000 => acc = acc + 1,
                x if 100_000 <= x && x < 1_000_000 => acc = acc + 1,
                _ => {}
            }
        }
        return acc;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::find_numbers(vec![12, 345, 2, 6, 7896]), 2);
        assert_eq!(Solution::find_numbers(vec![555, 901, 482, 1771]), 1);
    }
}
