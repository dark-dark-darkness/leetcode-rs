use std::collections::HashMap;

struct Solution;

// 1631. 最小体力消耗路径
// https://leetcode.cn/problems/path-with-minimum-effort

impl Solution {
    pub fn minimum_effort_path(heights: Vec<Vec<i32>>) -> i32 {
        use std::cmp::max;
        const INF: i32 = 0x3f3f3f3f;
        let n = heights.len();
        let m = heights[0].len();
        let k = n * m;
        let mut e: Vec<Vec<(i32, i32)>> = vec![vec![]; k];
        for i in 0..n {
            for j in 0..m {
                if j != 0 {
                    e[i * m + j].push((
                        (i * m + j - 1) as i32,
                        (heights[i][j] - heights[i][j - 1]).abs(),
                    ));
                }
                if j + 1 != m {
                    e[i * m + j].push((
                        (i * m + j + 1) as i32,
                        (heights[i][j] - heights[i][j + 1]).abs(),
                    ));
                }
                if i != 0 {
                    e[i * m + j].push((
                        ((i - 1) * m + j) as i32,
                        (heights[i][j] - heights[i - 1][j]).abs(),
                    ));
                }
                if i + 1 != n {
                    e[i * m + j].push((
                        ((i + 1) * m + j) as i32,
                        (heights[i][j] - heights[i + 1][j]).abs(),
                    ));
                }
            }
        }
        let mut f = true;
        let mut dis: Vec<i32> = vec![INF; k];
        dis[0] = 0;
        while f {
            f = false;
            for i in 0..k {
                let d = dis[i];
                for &(j, w) in &e[i] {
                    if max(d, w) < dis[j as usize] {
                        dis[j as usize] = max(d, w);
                        f = true;
                    }
                }
            }
        }
        dis[k - 1]
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn case_1() {
        let heights = [[1, 2, 2], [3, 8, 2], [5, 3, 5]];
        let right = 2;

        let result = super::Solution::minimum_effort_path(heights.map(|x| x.to_vec()).to_vec());

        assert_eq!(result, right);
    }

    #[test]
    fn case_2() {
        let heights = [[1, 2, 3], [3, 8, 4], [5, 3, 5]];
        let right = 1;

        let result = super::Solution::minimum_effort_path(heights.map(|x| x.to_vec()).to_vec());

        assert_eq!(result, right);
    }

    #[test]
    fn case_3() {
        let heights = [
            [1, 2, 1, 1, 1],
            [1, 2, 1, 2, 1],
            [1, 2, 1, 2, 1],
            [1, 2, 1, 2, 1],
            [1, 1, 1, 2, 1],
        ];
        let right = 0;

        let result = super::Solution::minimum_effort_path(heights.map(|x| x.to_vec()).to_vec());

        assert_eq!(result, right);
    }
}
