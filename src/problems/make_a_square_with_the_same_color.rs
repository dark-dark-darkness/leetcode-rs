struct Solution;

// 3127. 构造相同颜色的正方形
// https://leetcode.cn/problems/make-a-square-with-the-same-color

impl Solution {
    pub fn can_make_square(grid: Vec<Vec<char>>) -> bool {
        let len = grid.len();

        for x in 0..len - 1 {
            for y in 0..grid[x].len() - 1 {
                let arr = [
                    grid[x][y],
                    grid[x + 1][y],
                    grid[x][y + 1],
                    grid[x + 1][y + 1],
                ];
                let c = arr.iter().filter(|&&x| x == 'W').count();
                if c == 1 || c == 3 || c == 0 || c == 4 {
                    return true;
                }
            }
        }

        false
    }
}
