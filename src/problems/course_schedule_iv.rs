use std::{collections::HashSet, hash::Hash};

use crate::Solution;

// 1462. 课程表 IV
// https://leetcode.cn/problems/course-schedule-iv/

impl Solution {
    pub fn check_if_prerequisite(
        num_courses: i32,
        prerequisites: Vec<Vec<i32>>,
        queries: Vec<Vec<i32>>,
    ) -> Vec<bool> {
        let num_courses = num_courses as usize;
        let mut grid = vec![vec![false; num_courses]; num_courses];

        prerequisites
            .iter()
            .for_each(|t| grid[t[0] as usize][t[1] as usize] = true);

        for k in 0..num_courses {
            for i in 0..num_courses {
                for j in 0..num_courses {
                    grid[i][j] = grid[i][j] || grid[i][k] && grid[k][j];
                }
            }
        }

        queries
            .iter()
            .map(|x| grid[x[0] as usize][x[1] as usize])
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn case_1() {
        let num_courses = 2;
        let prerequisites = [[1, 0]];
        let queries = [[0, 1], [1, 0]];

        let right = [false, true];

        let result = Solution::check_if_prerequisite(
            num_courses,
            prerequisites.map(|x| x.into()).into(),
            queries.map(|x| x.into()).into(),
        );

        assert_eq!(result, right);
    }

    #[test]
    fn case_2() {
        let num_courses = 2;
        let prerequisites: [[i32; 0]; 0] = [];
        let queries = [[1, 0], [0, 1]];

        let right = [false, false];

        let result = Solution::check_if_prerequisite(
            num_courses,
            prerequisites.map(|x| x.into()).into(),
            queries.map(|x| x.into()).into(),
        );

        assert_eq!(result, right);
    }
}
