// https://leetcode.com/problems/rank-transform-of-an-array/

use std::collections::HashMap;

pub struct Solution;

// Time complexity:     O(n log(n))
// Space complexity:    O(n)
impl Solution {
    pub fn array_rank_transform(arr: Vec<i32>) -> Vec<i32> {
        let mut sorted = arr.clone();
        sorted.sort();

        let mut map: HashMap<i32, i32> = HashMap::new();
        let mut rank = 0;

        for val in sorted {
            map.entry(val).or_insert_with(|| {
                rank += 1;
                rank
            });
        }

        arr.iter().map(|&val| *map.get(&val).unwrap()).collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rank_transform_of_an_array_example_1() {
        let input = vec![40, 10, 20, 30];
        let exp_output = vec![4, 1, 2, 3];
        assert_eq!(Solution::array_rank_transform(input), exp_output);
    }

    #[test]
    fn test_rank_transform_of_an_array_example_2() {
        let input = vec![100, 100, 100];
        let exp_output = vec![1, 1, 1];
        assert_eq!(Solution::array_rank_transform(input), exp_output);
    }

    #[test]
    fn test_rank_transform_of_an_array_example_3() {
        let input = vec![37, 12, 28, 9, 100, 56, 80, 5, 12];
        let exp_output = vec![5, 3, 4, 2, 8, 6, 7, 1, 3];
        assert_eq!(Solution::array_rank_transform(input), exp_output);
    }
}
