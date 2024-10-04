// https://leetcode.com/problems/divide-players-into-teams-of-equal-skill/

pub struct Solution;

// Time complexity:     O(n)
// Space complexity:    O(n)
impl Solution {
    pub fn divide_players(skill: Vec<i32>) -> i64 {
        let total_skill: i32 = skill.iter().sum();
        let num_teams = skill.len() as i32 / 2;

        if total_skill % num_teams != 0 {
            return -1;
        }

        let target_skill = (total_skill / num_teams) as usize;

        let mut buckets = vec![0; target_skill];
        let mut total_chemistry: i64 = 0;

        for v in skill {
            buckets[v as usize % target_skill] += 1;
        }

        // check all items have symmetrical bucket matches
        for i in 1..=target_skill / 2 {
            let complement = target_skill - i;
            // Special check for single middle bucket - must be even because matches itself
            if i == complement {
                if buckets[i] % 2 != 0 {
                    return -1;
                }
                // halved because matches are from the same bucket
                total_chemistry += (i as i64 * complement as i64) * buckets[i] / 2
            } else {
                // If bucket size doesn't match size of complement bucket, perfect matches impossible
                if buckets[i] != buckets[complement] {
                    return -1;
                }
                // not halved because matches come from another bucket
                total_chemistry += (i as i64 * complement as i64) * buckets[i];
            }
        }

        total_chemistry
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divide_players_example_1() {
        let input = vec![3, 2, 5, 1, 3, 4];
        let exp_output = 22;
        assert_eq!(Solution::divide_players(input), exp_output);
    }

    #[test]
    fn test_divide_players_example_2() {
        let input = vec![3, 4];
        let exp_output = 12;
        assert_eq!(Solution::divide_players(input), exp_output);
    }

    #[test]
    fn test_divide_players_example_3() {
        let input = vec![1, 1, 2, 3];
        let exp_output = -1;
        assert_eq!(Solution::divide_players(input), exp_output);
    }
}
