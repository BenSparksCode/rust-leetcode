// https://leetcode.com/problems/permutation-in-string/

struct Solution;

// Time complexity:     O(n)
// Space complexity:    O(1)
// where n = s2.len()
impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let s1_len = s1.len();

        // s2 cannot contain a permutation of a larger string
        if s1_len > s2.len() {
            return false;
        }

        let mut current_chars = Self::count_chars(&s2[0..s1_len]);
        let target_chars = Self::count_chars(&s1);
        let a_offset = 'a' as usize;

        // check if start slice of s2 matches, otherwise move into loop and char counting
        if current_chars == target_chars {
            return true;
        }

        for i in 0..s2.len() - s1_len {
            // decrement count of offloaded char s2[i - 1]
            current_chars[s2.as_bytes()[i] as usize - a_offset] -= 1;

            // increment count of new char s2[i + s1_len]
            current_chars[s2.as_bytes()[i + s1_len] as usize - a_offset] += 1;

            if current_chars == target_chars {
                return true;
            }
        }

        // If we get here, all candidates in s2 have been checked - no matches
        false
    }

    // Full char count of string only done once for s1, and once at start of s2 iteration.
    // Returns a 26-length array of the counts of the a - z chars of the input string.
    fn count_chars(s: &str) -> [i32; 26] {
        let mut chars = [0; 26];
        let a_offset = 'a' as usize;

        for c in s.chars() {
            let idx = (c as usize) - a_offset;
            chars[idx] += 1;
        }

        chars
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permutation_in_string_example_1() {
        let s1 = String::from("ab");
        let s2 = String::from("eidbaooo");
        let expected = true;
        assert_eq!(Solution::check_inclusion(s1, s2), expected);
    }

    #[test]
    fn test_permutation_in_string_example_2() {
        let s1 = String::from("ab");
        let s2 = String::from("eidboaoo");
        let expected = false;
        assert_eq!(Solution::check_inclusion(s1, s2), expected);
    }

    #[test]
    fn test_permutation_in_string_example_3() {
        let s1 = String::from("adc");
        let s2 = String::from("dcda");
        let expected = true;
        assert_eq!(Solution::check_inclusion(s1, s2), expected);
    }
}
