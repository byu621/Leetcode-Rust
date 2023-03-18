pub struct Solution {}

use std::collections::HashMap;
impl Solution {
    fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        for (index, num) in nums.iter().enumerate() {
            let option = map.get(&(target - num));
            match option {
                None => {
                    map.insert(num, index);
                }
                Some(t) => {
                    return vec![*t as i32, index as i32];
                }
            }
        }
        return vec![];
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        assert_eq!(vec![0, 1], Solution::two_sum(vec![2, 7, 11, 15], 9));
    }

    #[test]
    fn test_2() {
        assert_eq!(vec![1, 2], Solution::two_sum(vec![3, 2, 4], 6));
    }
}
