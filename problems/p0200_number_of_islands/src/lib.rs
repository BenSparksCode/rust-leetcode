// https://leetcode.com/problems/number-of-islands/

use std::collections::HashSet;

struct Solution {}

// TODO fix - need proper path traversal, not just left > right, top > bottom. See last test case, bottom left counts as false additional island

impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        // Each node will have an id of (x + 5*y)
        let mut set: HashSet<usize> = HashSet::new();
        let width: usize = grid[0].len();
        let max: usize = grid.len() * grid[0].len();
        let mut count: i32 = 0;

        for (y, row) in grid.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                let c_id = y * width + x + 1; // starts at 1

                // Check if c is a candidate (1 and not explored yet)
                if *c == '1' && !set.contains(&c_id) {
                    if Solution::is_new_island(&set, c_id, width, max) {
                        count += 1;
                    }

                    // Mark c as explored
                    set.insert(c_id);
                }
            }
        }

        count
    }

    // Check if coord is a new island. If any valid squares above, below, left, or right, are marked as explored, c is part of an already counted island, so return false. Otherwise return true.
    pub fn is_new_island(set: &HashSet<usize>, c_id: usize, width: usize, max: usize) -> bool {
        // Above
        if c_id > width && set.contains(&(c_id - width)) {
            return false;
        }
        // Below
        if c_id <= max - width && set.contains(&(c_id + width)) {
            return false;
        }
        // Left
        if c_id % width != 1 && set.contains(&(c_id - 1)) {
            return false;
        }
        // Right
        if c_id % width != 0 && set.contains(&(c_id + 1)) {
            return false;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_of_islands_example_1() {
        let input: Vec<Vec<char>> = vec![
            vec!['1', '1', '1', '1', '0'],
            vec!['1', '1', '0', '1', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '0', '0', '0'],
        ];
        let expected = 1;
        assert_eq!(Solution::num_islands(input), expected);
    }

    #[test]
    fn test_number_of_islands_example_2() {
        let input: Vec<Vec<char>> = vec![
            vec!['1', '1', '0', '0', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '1', '0', '0'],
            vec!['0', '0', '0', '1', '1'],
        ];
        let expected = 3;
        assert_eq!(Solution::num_islands(input), expected);
    }

    #[test]
    fn test_number_of_islands_example_3() {
        let input: Vec<Vec<char>> = vec![vec!['1'], vec!['1']];
        let expected = 1;
        assert_eq!(Solution::num_islands(input), expected);
    }

    #[test]
    fn test_number_of_islands_example_4() {
        let input: Vec<Vec<char>> = vec![
            vec!['1', '1', '1'],
            vec!['0', '1', '0'],
            vec!['1', '1', '1'],
        ];
        let expected = 1;
        assert_eq!(Solution::num_islands(input), expected);
    }
}
