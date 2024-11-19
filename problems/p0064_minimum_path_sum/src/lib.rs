// https://leetcode.com/problems/minimum-path-sum/

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

struct Solution {}

impl Solution {
    // Solution using Dijkstra search
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let mut dists: HashMap<usize, usize> = HashMap::new();
        let mut heap: BinaryHeap<Reverse<(usize, usize, usize)>> = BinaryHeap::new();

        Solution::explore_dijkstra(&grid, &mut dists, &mut heap);

        *dists.get(&(grid.len() * grid[0].len() - 1)).unwrap() as i32
    }

    // Solution using DFS search
    pub fn min_path_sum_dfs(grid: Vec<Vec<i32>>) -> i32 {
        let mut dists: HashMap<usize, usize> = HashMap::new();

        Solution::explore_dfs(&grid, &mut dists, 0, 0, 0);

        *dists.get(&(grid.len() * grid[0].len() - 1)).unwrap() as i32
    }

    // Dijkstra's Algorithm implementation
    pub fn explore_dijkstra(
        grid: &Vec<Vec<i32>>,
        dists: &mut HashMap<usize, usize>,
        heap: &mut BinaryHeap<Reverse<(usize, usize, usize)>>,
    ) {
        // Heap Item format: (dist, x, y)

        // Add starting tile to heap and dists map
        heap.push(Reverse((grid[0][0] as usize, 0, 0)));
        dists.insert(0, grid[0][0] as usize);

        while let Some(Reverse((curr_dist, x, y))) = heap.pop() {
            let p = y * grid[0].len() + x;

            if curr_dist > *dists.get(&p).unwrap_or(&usize::MAX) {
                continue;
            }

            // Then add adjacent nodes with their distances to the heap:

            // Down
            if y < grid.len() - 1 {
                let (x_new, y_new) = (x, y + 1);

                let dist_new = curr_dist + grid[y_new][x_new] as usize;
                let p_new = y_new * grid[0].len() + x_new;
                if dist_new < *dists.get(&p_new).unwrap_or(&usize::MAX) {
                    dists.insert(p_new, dist_new);
                    heap.push(Reverse((dist_new, x_new, y_new)));
                }
            }
            // Right
            if x < grid[0].len() - 1 {
                let (x_new, y_new) = (x + 1, y);

                let dist_new = curr_dist + grid[y_new][x_new] as usize;
                let p_new = y_new * grid[0].len() + x_new;
                if dist_new < *dists.get(&p_new).unwrap_or(&usize::MAX) {
                    dists.insert(p_new, dist_new);
                    heap.push(Reverse((dist_new, x_new, y_new)));
                }
            }

            // NOTE: Instructions say only DOWN or RIGHT moves allowed, so UP and LEFT are disabled
        }
    }

    // NOTE: This explore implementation is NOT Dijkstra's Algorithm. This is DFS.
    // My first attempt using recursive exploring and a HashMap of shortest distances.
    // x and y are the coords of current location on the grid.
    // dist is the total distance traveled so far to reach this point on the grid, not including this point.
    pub fn explore_dfs(
        grid: &Vec<Vec<i32>>,
        dists: &mut HashMap<usize, usize>,
        x: usize,
        y: usize,
        dist: usize,
    ) {
        let p = y * grid[0].len() + x;
        let new_dist = dist + grid[y][x] as usize;

        // If point has already been explored, and has a lower dist than current path, stop exploring
        if dists.contains_key(&p) && new_dist >= *dists.get(&p).unwrap() {
            return;
        }

        // Set new_dist as the new lowest value for point p
        dists.insert(p, new_dist);
        // Then continue exploring in all directions, if not at the edge of the grid
        // NOTE: By putting RIGHT and DOWN explores at the top, it should bias to finding the quickest path from top left to bottom right.

        // Right
        if x < grid[0].len() - 1 {
            Solution::explore_dfs(grid, dists, x + 1, y, new_dist);
        }
        // Down
        if y < grid.len() - 1 {
            Solution::explore_dfs(grid, dists, x, y + 1, new_dist);
        }

        // NOTE: Instructions say only DOWN or RIGHT moves allowed, so UP and LEFT are disabled
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_path_sum_example_1() {
        let input: Vec<Vec<i32>> = vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]];
        let expected = 7;
        assert_eq!(Solution::min_path_sum(input), expected);
    }

    #[test]
    fn test_min_path_sum_example_2() {
        let input: Vec<Vec<i32>> = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let expected = 12;
        assert_eq!(Solution::min_path_sum(input), expected);
    }

    #[test]
    fn test_min_path_sum_example_3() {
        let input: Vec<Vec<i32>> = vec![
            vec![5, 4, 2, 9, 6, 0, 3, 5, 1, 4, 9, 8, 4, 9, 7, 5, 1],
            vec![3, 4, 9, 2, 9, 9, 0, 9, 7, 9, 4, 7, 8, 4, 4, 5, 8],
            vec![6, 1, 8, 9, 8, 0, 3, 7, 0, 9, 8, 7, 4, 9, 2, 0, 1],
            vec![4, 0, 0, 5, 1, 7, 4, 7, 6, 4, 1, 0, 1, 0, 6, 2, 8],
            vec![7, 2, 0, 2, 9, 3, 4, 7, 0, 8, 9, 5, 9, 0, 1, 1, 0],
            vec![8, 2, 9, 4, 9, 7, 9, 3, 7, 0, 3, 6, 5, 3, 5, 9, 6],
            vec![8, 9, 9, 2, 6, 1, 2, 5, 8, 3, 7, 0, 4, 9, 8, 8, 8],
            vec![5, 8, 5, 4, 1, 5, 6, 6, 3, 3, 1, 8, 3, 9, 6, 4, 8],
            vec![0, 2, 2, 3, 0, 2, 6, 7, 2, 3, 7, 3, 1, 5, 8, 1, 3],
            vec![4, 4, 0, 2, 0, 3, 8, 4, 1, 3, 3, 0, 7, 4, 2, 9, 8],
            vec![5, 9, 0, 4, 7, 5, 7, 6, 0, 8, 3, 0, 0, 6, 6, 6, 8],
            vec![0, 7, 1, 8, 3, 5, 1, 8, 7, 0, 2, 9, 2, 2, 7, 1, 5],
            vec![1, 0, 0, 0, 6, 2, 0, 0, 2, 2, 8, 0, 9, 7, 0, 8, 0],
            vec![1, 1, 7, 2, 9, 6, 5, 4, 8, 7, 8, 5, 0, 3, 8, 1, 5],
            vec![8, 9, 7, 8, 1, 1, 3, 0, 1, 2, 9, 4, 0, 1, 5, 3, 1],
            vec![9, 2, 7, 4, 8, 7, 3, 9, 2, 4, 2, 2, 7, 8, 2, 6, 7],
            vec![3, 8, 1, 6, 0, 4, 8, 9, 8, 0, 2, 5, 3, 5, 5, 7, 5],
            vec![1, 8, 2, 5, 7, 7, 1, 9, 9, 8, 9, 2, 4, 9, 5, 4, 0],
            vec![3, 4, 4, 1, 5, 3, 3, 8, 8, 6, 3, 5, 3, 8, 7, 1, 3],
        ];
        let expected = 82;
        assert_eq!(Solution::min_path_sum(input), expected);
    }

    #[test]
    fn test_min_path_sum_example_4() {
        let input: Vec<Vec<i32>> = vec![vec![0, 0], vec![0, 0]];
        let expected = 0;
        assert_eq!(Solution::min_path_sum(input), expected);
    }
}
