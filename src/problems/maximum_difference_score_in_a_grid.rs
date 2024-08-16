struct Solution;
// 3148. 矩阵中的最大得分
// https://leetcode.cn/problems/maximum-difference-score-in-a-grid

impl Solution {
    pub fn max_score(grid: Vec<Vec<i32>>) -> i32 {
        let mut dp = vec![vec![i32::MAX; grid[0].len()]; grid.len()];
        dp[0][0] = i32::MAX;

        for x in 0..dp.len() {
            for y in 0..dp[x].len() {
                if x == 0 && y == 0 {
                    continue;
                }
                if x == 0 {
                    dp[0][y] = i32::min(dp[0][y - 1], grid[0][y - 1])
                } else if y == 0 {
                    dp[x][0] = i32::min(dp[x - 1][0], grid[x - 1][0])
                } else {
                    dp[x][y] = dp[x - 1][y]
                        .min(dp[x][y - 1])
                        .min(grid[x - 1][y])
                        .min(grid[x][y - 1]);
                }
            }
        }

        grid.iter()
            .flatten()
            .zip(dp.iter().flatten())
            .map(|(a, b)| a - b)
            .max()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn case_1() {
        let grid = [[9, 5, 7, 3], [8, 9, 6, 1], [6, 7, 14, 3], [2, 5, 3, 1]];
        let right = 9;
        let result = Solution::max_score(grid.map(|x| x.to_vec()).to_vec());
        assert_eq!(result, right);
    }

    #[test]
    fn case_2() {
        let grid = [[4, 3, 2], [3, 2, 1]];
        let right = -1;
        let result = Solution::max_score(grid.map(|x| x.to_vec()).to_vec());
        assert_eq!(result, right);
    }

    #[test]
    fn case_3() {
        let grid = [[7, 10, 10], [4, 9, 3], [5, 5, 3], [9, 9, 6], [5, 3, 7]];
        let right = 5;
        let result = Solution::max_score(grid.map(|x| x.to_vec()).to_vec());
        assert_eq!(result, right);
    }

    #[test]
    fn case_4() {
        let grid = [[9, 10, 9], [4, 1, 2], [3, 4, 9]];
        let right = 8;
        let result = Solution::max_score(grid.map(|x| x.to_vec()).to_vec());
        assert_eq!(result, right);
    }
}
