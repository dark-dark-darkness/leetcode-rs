struct Solution;
// 3142. 判断矩阵是否满足条件
// https://leetcode.cn/problems/check-if-grid-satisfies-conditions/
impl Solution {
    pub fn satisfies_conditions(grid: Vec<Vec<i32>>) -> bool {
        for i in 0..grid[0].len() - 1 {
            if grid[0][i] == grid[0][i + 1] {
                return false;
            }
        }
        for i in 0..grid[0].len() {
            let v = grid[0][i];
            for row in grid.iter() {
                if row[i] != v {
                    return false;
                }
            }
        }
        true
    }
}
