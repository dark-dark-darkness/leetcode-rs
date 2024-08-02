struct Solution;

// 3128. 直角三角形
// https://leetcode.cn/problems/right-triangles

impl Solution {
    pub fn number_of_right_triangles(grid: Vec<Vec<i32>>) -> i64 {
        let mut cols = vec![0; grid[0].len()];

        for row in grid.iter() {
            for (y, col) in row.iter().enumerate() {
                cols[y] += col;
            }
        }
        let mut res: i64 = 0;
        for (x, row) in grid.iter().enumerate() {
            let row_sum: i32 = row.iter().sum();
            for (y, &col) in grid[x].iter().enumerate() {
                if col == 1 {
                    res += (row_sum - 1) as i64 * (cols[y] - 1) as i64;
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
