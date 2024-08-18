struct Solution;
impl Solution {
    pub fn count_battleships(board: Vec<Vec<char>>) -> i32 {
        let mut result = 0;
        for (x, row) in board.iter().enumerate() {
            for (y, &v) in row.iter().enumerate() {
                if v == 'X' && (y == 0 || row[y - 1] != 'X') && (x == 0 || board[x - 1][y] != 'X') {
                    result += 1;
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn case_1() {
        let board = [
            ['X', '.', '.', 'X'],
            ['.', '.', '.', 'X'],
            ['.', '.', '.', 'X'],
        ];
        let right = 2;
        let result = Solution::count_battleships(board.map(|x| x.to_vec()).to_vec());

        assert_eq!(result, right)
    }

    #[test]
    fn case_2() {
        let board = [['.']];
        let right = 0;
        let result = Solution::count_battleships(board.map(|x| x.to_vec()).to_vec());

        assert_eq!(result, right)
    }

    #[test]
    fn case_3() {
        let board = [['X']];
        let right = 1;
        let result = Solution::count_battleships(board.map(|x| x.to_vec()).to_vec());

        assert_eq!(result, right)
    }
}
