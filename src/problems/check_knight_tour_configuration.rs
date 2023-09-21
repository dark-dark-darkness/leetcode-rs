struct Solution;

// 2596. 检查骑士巡视方案
// https://leetcode.cn/problems/check-knight-tour-configuration/

impl Solution {
    pub fn check_valid_grid(grid: Vec<Vec<i32>>) -> bool {
        if grid[0][0] != 0 {
            return false;
        }

        let mut p = (0, 0);
        let n = grid.len();

        loop {
            let (x, y) = p;
            if grid[x][y] as usize == n * n - 1 {
                return true;
            }
            let next = can_move_points(p, n)
                .into_iter()
                .find(|&(n_x, n_y)| grid[n_x][n_y] == grid[x][y] + 1);
            match next {
                Some(next) => p = next,
                None => {
                    return false;
                }
            }
        }
    }
}

fn can_move_points((x, y): (usize, usize), n: usize) -> Vec<(usize, usize)> {
    let x = x as i32;
    let y = y as i32;
    let n = n as i32;
    [
        (x - 1, y - 2),
        (x - 1, y + 2),
        (x - 2, y - 1),
        (x - 2, y + 1),
        (x + 1, y - 2),
        (x + 1, y + 2),
        (x + 2, y - 1),
        (x + 2, y + 1),
    ]
    .iter()
    .filter(|&&(a, b)| a >= 0 && b >= 0 && a < n && b < n)
    .map(|&(a, b)| (a as usize, b as usize))
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let grid = [
            [0, 11, 16, 5, 20],
            [17, 4, 19, 10, 15],
            [12, 1, 8, 21, 6],
            [3, 18, 23, 14, 9],
            [24, 13, 2, 7, 22],
        ];
        let right = true;

        let result = Solution::check_valid_grid(grid.map(|x| x.into()).into());

        assert_eq!(result, right);
    }

    #[test]
    fn case_2() {
        let grid = [[0, 3, 6], [5, 8, 1], [2, 7, 4]];
        let right = false;

        let result = Solution::check_valid_grid(grid.map(|x| x.into()).into());

        assert_eq!(result, right);
    }

    #[test]
    fn case_3() {
        let grid = [
            [24, 11, 22, 17, 4],
            [21, 16, 5, 12, 9],
            [6, 23, 10, 3, 18],
            [15, 20, 1, 8, 13],
            [0, 7, 14, 19, 2],
        ];
        let right = false;

        let result = Solution::check_valid_grid(grid.map(|x| x.into()).into());

        assert_eq!(result, right);
    }
}
