struct Solution;

impl Solution {
    pub fn min_falling_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        // rcd[i] 列下标为i的数为结尾的最小和
        let mut rcd = grid[0].clone();

        for row in grid.iter().skip(1) {
            let mut new_rcd = Vec::with_capacity(row.len());
            // 优化点：先找出一行中最小的下标和值，只有这个最小下标位置的最小值需要重新找到倒数第二小的，其他的位置最小值一定就是这个最小值
            let (min_idx, min_val) = rcd.iter().enumerate().min_by(|a, b| a.1.cmp(b.1)).unwrap();
            new_rcd.reserve_exact(rcd.len());
            for (idx, v) in row.iter().enumerate() {
                if idx != min_idx {
                    // 不是最小值所对应的下标，直接用最小值
                    new_rcd.push(min_val + v);
                } else {
                    // 用除了最小值之外的第二小的值
                    let m = rcd
                        .iter()
                        .enumerate()
                        .filter(|(i, _)| *i != idx)
                        .map(|x| x.1)
                        .min();
                    if let Some(m) = m {
                        new_rcd.push(m + v);
                    }
                }
            }
            rcd = new_rcd;
        }
        rcd.into_iter().min().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let grid = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
        let right = 13;

        let result = Solution::min_falling_path_sum(grid.iter().map(|x| x.to_vec()).collect());

        assert_eq!(result, right);
    }

    #[test]
    fn case_2() {
        let grid = [[7]];
        let right = 7;

        let result = Solution::min_falling_path_sum(grid.iter().map(|x| x.to_vec()).collect());

        assert_eq!(result, right);
    }

    #[test]
    fn case_3() {
        let grid = [
            [-73, 61, 43, -48, -36],
            [3, 30, 27, 57, 10],
            [96, -76, 84, 59, -15],
            [5, -49, 76, 31, -7],
            [97, 91, 61, -46, 67],
        ];
        let right = -192;

        let result = Solution::min_falling_path_sum(grid.iter().map(|x| x.to_vec()).collect());

        assert_eq!(result, right);
    }
}
