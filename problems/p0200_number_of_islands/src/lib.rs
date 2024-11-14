// https://leetcode.com/problems/number-of-islands/

use std::collections::HashSet;

struct Solution {}

// Time complexity:     O(n * m)
// Space complexity:    O(n * m)
// where {n, m} are {length, width} of grid
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
                    Solution::explore(&grid, &mut set, x, y);

                    count += 1;
                }
            }
        }

        count
    }

    pub fn explore(grid: &Vec<Vec<char>>, set: &mut HashSet<usize>, x: usize, y: usize) {
        let width: usize = grid[0].len();
        let p: usize = y * width + x + 1;

        // If we hit water or an explored coord, stop exploring
        if grid[y][x] == '0' || set.contains(&p) {
            return;
        }

        // Mark current location as explored
        set.insert(p);

        // Above
        if y > 0 {
            Solution::explore(grid, set, x, y - 1);
        }
        // Below
        if y < grid.len() - 1 {
            Solution::explore(grid, set, x, y + 1);
        }
        // Left
        if x > 0 {
            Solution::explore(grid, set, x - 1, y);
        }
        // Right
        if x < width - 1 {
            Solution::explore(grid, set, x + 1, y);
        }
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
