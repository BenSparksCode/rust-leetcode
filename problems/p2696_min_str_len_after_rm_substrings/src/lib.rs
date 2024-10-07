// https://leetcode.com/problems/minimum-string-length-after-removing-substrings/

struct Solution {}

// Time complexity:     O(n^2)
// Space complexity:    O(n)
impl Solution {
    pub fn min_length(mut s: String) -> i32 {
        loop {
            let len_last = s.len();
            s = s.replace("AB", "").replace("CD", "");
            if s.len() == len_last {
                break;
            }
        }

        s.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_str_len_after_rm_substrings_example_1() {
        let s = String::from("ABFCACDB");
        let expected = 2;
        assert_eq!(Solution::min_length(s), expected);
    }

    #[test]
    fn test_min_str_len_after_rm_substrings_example_2() {
        let s = String::from("ACBBD");
        let expected = 5;
        assert_eq!(Solution::min_length(s), expected);
    }
}
