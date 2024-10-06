// https://leetcode.com/problems/sentence-similarity-iii/

struct Solution {}

// Time complexity:     O(n)
// Space complexity:    O(m + k)
// where n = shortest sentence char length
// and m and k = word length of sentences
impl Solution {
    pub fn are_sentences_similar(sentence1: String, sentence2: String) -> bool {
        if sentence1 == sentence2 {
            return true;
        }

        let short: Vec<&str>;
        let long: Vec<&str>;
        let mut match_count: usize = 0;

        // Convert sentences into short and long vectors of words
        if sentence1.len() == sentence2.len() {
            // If same char length, but not exact match, cannot be similar
            return false;
        } else if sentence1.len() > sentence2.len() {
            short = sentence2.split_whitespace().collect();
            long = sentence1.split_whitespace().collect();
        } else {
            short = sentence1.split_whitespace().collect();
            long = sentence2.split_whitespace().collect();
        }

        // First iterate matches left to right
        for (i, short_word) in short.iter().enumerate() {
            if *short_word == long[i] {
                match_count += 1
            } else {
                break;
            }
        }

        if match_count == short.len() {
            return true;
        }

        // Then iterate matches right to left
        for (i, short_word) in short.iter().rev().enumerate() {
            if *short_word == long[long.len() - 1 - i] {
                match_count += 1
            } else {
                break;
            }
        }

        // Only true if short has been exhausted and all words have matches
        match_count >= short.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sentence_similarity_3_example_1() {
        let s1 = String::from("My name is Haley");
        let s2 = String::from("My Haley");
        let expected = true;
        assert_eq!(Solution::are_sentences_similar(s1, s2), expected);
    }

    #[test]
    fn test_sentence_similarity_3_example_2() {
        let s1 = String::from("My name is Haley");
        let s2 = String::from("My Bob");
        let expected = false;
        assert_eq!(Solution::are_sentences_similar(s1, s2), expected);
    }

    #[test]
    fn test_sentence_similarity_3_example_3() {
        let s1 = String::from("of");
        let s2 = String::from("A lot of words");
        let expected = false;
        assert_eq!(Solution::are_sentences_similar(s1, s2), expected);
    }

    #[test]
    fn test_sentence_similarity_3_example_4() {
        let s1 = String::from("Eating right now");
        let s2 = String::from("Eating");
        let expected = true;
        assert_eq!(Solution::are_sentences_similar(s1, s2), expected);
    }

    #[test]
    fn test_sentence_similarity_3_example_5() {
        let s1 = String::from("A A A A A");
        let s2 = String::from("A A A");
        let expected = true;
        assert_eq!(Solution::are_sentences_similar(s1, s2), expected);
    }

    #[test]
    fn test_sentence_similarity_3_example_6() {
        let s1 = String::from("A");
        let s2 = String::from("A A A A A");
        let expected = true;
        assert_eq!(Solution::are_sentences_similar(s1, s2), expected);
    }

    #[test]
    fn test_sentence_similarity_3_example_7() {
        let s1 = String::from("A A AAa");
        let s2 = String::from("A AAa");
        let expected = true;
        assert_eq!(Solution::are_sentences_similar(s1, s2), expected);
    }
}
