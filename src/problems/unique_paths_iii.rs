use crate::Solution;

// https://leetcode.cn/problems/unique-paths-iii/

impl Solution {
    pub fn unique_paths_iii(grid: Vec<Vec<i32>>) -> i32 {
        let path: Vec<(usize, usize)> = vec![Solution::find_point(&grid, 1)];

        let path_len = grid.iter().flatten().filter(|&&x| x == 0 || x == 1).count();

        Solution::unique_paths_iii_rec(&grid, &path, path_len)
    }

    fn unique_paths_iii_rec(grid: &Vec<Vec<i32>>, path: &[(usize, usize)], path_len: usize) -> i32 {
        let mut acc = 0;
        let &(x, y) = path.last().unwrap();
        if x != 0 {
            acc += Solution::move_point(grid.clone(), path.to_owned(), (x - 1, y), path_len);
        }

        if x + 1 < grid.len() {
            acc += Solution::move_point(grid.clone(), path.to_owned(), (x + 1, y), path_len);
        }

        if y != 0 {
            acc += Solution::move_point(grid.clone(), path.to_owned(), (x, y - 1), path_len);
        }

        if y + 1 < grid[x].len() {
            acc += Solution::move_point(grid.clone(), path.to_owned(), (x, y + 1), path_len);
        }

        acc
    }

    fn move_point(
        mut grid: Vec<Vec<i32>>,
        mut path: Vec<(usize, usize)>,
        to @ (i, j): (usize, usize),
        path_len: usize,
    ) -> i32 {
        match grid[i][j] {
            0 => {
                grid[i][j] = -1;
                path.push(to);
                Solution::unique_paths_iii_rec(&grid, &path, path_len)
            }
            -1 | 1 => 0,
            2 => {
                if path.len() == path_len {
                    1
                } else {
                    0
                }
            }
            _ => unimplemented!(),
        }
    }
    #[allow(clippy::needless_range_loop)]
    fn find_point(grid: &Vec<Vec<i32>>, value: i32) -> (usize, usize) {
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] == value {
                    return (i, j);
                }
            }
        }
        (0, 0)
    }
}
#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn case_1() {
        let input = [[1, 0, 0, 0], [0, 0, 0, 0], [0, 0, 2, -1]];
        let right = 2;

        let result = Solution::unique_paths_iii(input.iter().map(|a| a.to_vec()).collect());

        assert_eq!(right, result);
    }

    #[test]
    fn case_2() {
        let input = [[1, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 2]];
        let right = 4;

        let result = Solution::unique_paths_iii(input.iter().map(|a| a.to_vec()).collect());

        assert_eq!(right, result);
    }

    #[test]
    fn case_3() {
        let input = [[0, 1], [2, 0]];
        let right = 0;

        let result = Solution::unique_paths_iii(input.iter().map(|a| a.to_vec()).collect());

        assert_eq!(right, result);
    }
}
