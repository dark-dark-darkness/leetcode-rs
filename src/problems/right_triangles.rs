struct Solution;

// 3128. 直角三角形
// https://leetcode.cn/problems/right-triangles

impl Solution {
    pub fn number_of_right_triangles(grid: Vec<Vec<i32>>) -> i64 {
        let mut col = vec![0; grid[0].len()];

        for x in 0..grid.len() {
            for y in 0..grid[x].len() {
                col[y] += grid[x][y];
            }
        }
        let mut res: i64 = 0;
        for x in 0..grid.len() {
            let row: i32 = grid[x].iter().sum();
            for y in 0..grid[x].len() {
                if grid[x][y] == 1 {
                    res += (row - 1) as i64 * (col[y] - 1) as i64;
                }
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn case_1() {
        let grid = [[0, 1, 0], [0, 1, 1], [0, 1, 0]];
        let right = 2;
        let result = Solution::number_of_right_triangles(grid.map(|x| x.to_vec()).to_vec());
        assert_eq!(result, right)
    }

    #[test]
    fn case_2() {
        let grid = [[1, 0, 0, 0], [0, 1, 0, 1], [1, 0, 0, 0]];
        let right = 0;
        let result = Solution::number_of_right_triangles(grid.map(|x| x.to_vec()).to_vec());
        assert_eq!(result, right)
    }

    #[test]
    fn case_3() {
        let grid = [[1, 0, 1], [1, 0, 0], [1, 0, 0]];
        let right = 2;
        let result = Solution::number_of_right_triangles(grid.map(|x| x.to_vec()).to_vec());
        assert_eq!(result, right)
    }
}
