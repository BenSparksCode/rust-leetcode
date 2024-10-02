// https://leetcode.com/problems/check-if-array-pairs-are-divisible-by-k/

pub struct Solution;

impl Solution {
    // Time complexity:     O(n^2)
    // Space complexity:    O(n)
    pub fn can_arrange_slow(arr: &Vec<i32>, k: i32) -> bool {
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

    // Time complexity:     O(n + k)
    // Space complexity:    O(k)
    pub fn can_arrange_fast(arr: &Vec<i32>, k: i32) -> bool {
        let mut buckets = vec![0; (k + 1) as usize];

        for val in arr {
            // rem_euclid avoids negative % result, only positive r
            let r = val.rem_euclid(k);
            buckets[r as usize] += 1;
        }

        // Check if 0 bucket is even
        if buckets[0] % 2 != 0 {
            return false;
        }

        // Check for all other buckets, each item has a match in a symmetrical bucket
        for i in 1..=(k / 2) {
            if i == k - i {
                // If k is even, middle bucket is own symmetrical bucket -> must be even
                if buckets[i as usize] % 2 != 0 {
                    return false;
                }
            } else {
                if buckets[i as usize] != buckets[(k - i) as usize] {
                    return false;
                }
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
        assert_eq!(Solution::can_arrange_slow(&arr, k), true, "slow failed");
        assert_eq!(Solution::can_arrange_fast(&arr, k), true, "fast failed");
    }

    #[test]
    fn test_check_array_pairs_divisible_by_k_example_2() {
        let arr = vec![1, 2, 3, 4, 5, 6];
        let k = 7;
        assert_eq!(Solution::can_arrange_slow(&arr, k), true, "slow failed");
        assert_eq!(Solution::can_arrange_fast(&arr, k), true, "fast failed");
    }

    #[test]
    fn test_check_array_pairs_divisible_by_k_example_3() {
        let arr = vec![1, 2, 3, 4, 5, 6];
        let k = 10;
        assert_eq!(Solution::can_arrange_slow(&arr, k), false, "slow failed");
        assert_eq!(Solution::can_arrange_fast(&arr, k), false, "fast failed");
    }

    #[test]
    fn test_check_array_pairs_divisible_by_k_example_4() {
        let arr = vec![5, 5, 1, 2, 3, 4];
        let k = 10;
        assert_eq!(Solution::can_arrange_slow(&arr, k), false, "slow failed");
        assert_eq!(Solution::can_arrange_fast(&arr, k), false, "fast failed");
    }

    #[test]
    fn test_check_array_pairs_divisible_by_k_example_5() {
        let arr = vec![1, 2, 3, 3, 4, 5];
        let k = 6;
        assert_eq!(Solution::can_arrange_slow(&arr, k), true, "slow failed");
        assert_eq!(Solution::can_arrange_fast(&arr, k), true, "fast failed");
    }

    #[test]
    fn test_check_array_pairs_divisible_by_k_example_6() {
        let arr = vec![-1, 1, -2, 2, -3, 3, -4, 4];
        let k = 3;
        assert_eq!(Solution::can_arrange_slow(&arr, k), true, "slow failed");
        assert_eq!(Solution::can_arrange_fast(&arr, k), true, "fast failed");
    }
}
