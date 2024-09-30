use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();

        for (i, num1) in nums.iter().enumerate() {
            match map.get(&num1) {
                Some(idx) => {
                    return vec![i as i32, *idx];
                }
                None => {
                    map.insert(target - num1, i as i32);
                }
            }
        }
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_eq_unordered(mut v1: Vec<i32>, mut v2: Vec<i32>) {
        v1.sort();
        v2.sort();
        assert_eq!(v1, v2);
    }

    #[test]
    fn test_two_sum_example_1() {
        assert_eq_unordered(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    }

    #[test]
    fn test_two_sum_example_2() {
        assert_eq_unordered(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    }
}
