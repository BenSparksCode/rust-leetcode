// https://leetcode.com/problems/check-if-array-pairs-are-divisible-by-k/

pub struct Solution;

impl Solution {
    // Time complexity:     O(n^2)
    // Space complexity:    O(n)
    pub fn can_arrange(arr: Vec<i32>, k: i32) -> bool {
        let mut used = vec![false; arr.len()];

        for (i, val1) in arr.iter().enumerate() {
            if used[i] {
                continue;
            }

            let mut pair_found = false;

            for (j, val2) in arr.iter().enumerate().skip(i + 1) {
                if used[j] {
                    continue;
                }

                // check the sum of the pair is divisible by k
                if (val1 + val2) % k == 0 {
                    used[i] = true;
                    used[j] = true;
                    pair_found = true;
                    break;
                }
            }
            if !pair_found {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_array_pairs_divisible_by_k_example_1() {
        let arr = vec![1, 2, 3, 4, 5, 10, 6, 7, 8, 9];
        let k = 5;
        assert_eq!(Solution::can_arrange(arr, k), true);
    }

    #[test]
    fn test_check_array_pairs_divisible_by_k_example_2() {
        let arr = vec![1, 2, 3, 4, 5, 6];
        let k = 7;
        assert_eq!(Solution::can_arrange(arr, k), true);
    }

    #[test]
    fn test_check_array_pairs_divisible_by_k_example_3() {
        let arr = vec![1, 2, 3, 4, 5, 6];
        let k = 10;
        assert_eq!(Solution::can_arrange(arr, k), false);
    }
}
