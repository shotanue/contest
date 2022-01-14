struct Solution {}

impl Solution {
    // https://leetcode.com/explore/learn/card/fun-with-arrays/525/inserting-items-into-an-array/3245/
    // https://leetcode.com/problems/duplicate-zeros/discuss/889298/Simple-Rust-implementation
    pub fn duplicate_zeros(arr: &mut Vec<i32>) {
        let mut index = 0;
        let total_len = arr.len();

        while index < total_len {
            if arr[index] == 0 && (index + 1) <= arr.len() {
                arr.insert(index + 1, 0);
                arr.remove(arr.len() - 1);
                index += 2;
                continue;
            }

            index += 1;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_1() {
        let mut a = vec![1, 0, 2, 3, 0, 4, 5, 0];
        Solution::duplicate_zeros(&mut a);
        assert_eq!(a[..], [1, 0, 0, 2, 3, 0, 0, 4]);
        // assert_eq!(Solution::duplicate_zeros(vec![555, 901, 482, 1771]), 1);
    }
}
