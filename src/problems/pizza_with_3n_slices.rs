use crate::Solution;
use std::collections::HashMap;

//https://leetcode.cn/problems/pizza-with-3n-slices/

impl Solution {
    pub fn max_size_slices(slices: Vec<i32>) -> i32 {
        fn calc(s: &[i32]) -> i32 {
            let mut pre = vec![0; s.len() + 2];
            for _ in 0..s.len() / 3 + 1 {
                let mut cur = vec![0; s.len() + 2];
                for i in 2..pre.len() {
                    cur[i] = cur[i - 1].max(pre[i - 2] + s[i - 2]);
                }
                pre = cur;
            }
            pre[s.len() + 1]
        }
        calc(&slices[0..slices.len() - 1]).max(calc(&slices[1..]))
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn case_1() {
        let slices = [1, 2, 3, 4, 5, 6];

        let right = 10;

        let result = Solution::max_size_slices(slices.to_vec());

        assert_eq!(result, right);
    }

    #[test]
    fn case_2() {
        let slices = [8, 9, 8, 6, 1, 1];

        let right = 16;

        let result = Solution::max_size_slices(slices.to_vec());

        assert_eq!(result, right);
    }

    #[test]
    fn case_3() {
        let slices = [
            2, 4, 3, 1, 5, 9, 4, 4, 5, 6, 5, 7, 6, 3, 10, 6, 8, 8, 8, 3, 5, 5, 8, 1, 8, 2, 6, 7, 2,
            1, 3, 4, 7, 10, 8, 7, 9, 5, 1, 1, 9, 10, 3, 10, 5, 10, 5, 2, 4, 6, 6, 1, 9, 4, 8, 2, 1,
        ];

        let right = 153;

        let result = Solution::max_size_slices(slices.to_vec());

        assert_eq!(result, right);
    }
}
