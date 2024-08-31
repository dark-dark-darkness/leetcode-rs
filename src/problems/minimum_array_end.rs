struct Solution;
// 3133. 数组最后一个元素的最小值
// https://leetcode.cn/problems/minimum-array-end/

impl Solution {
    pub fn min_end(n: i32, x: i32) -> i64 {
        let bit_count = 64 - n.leading_zeros() - x.leading_zeros();
        let mut res = x as i64;
        let mut j = 0;
        let n = (n - 1) as i64;
        for i in 0..bit_count {
            if ((res >> i) & 1) == 0 {
                if ((n >> j) & 1) != 0 {
                    res |= 1 << i;
                }
                j += 1;
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
        let (n, x) = (3, 4);
        let right = 6;
        let result = Solution::min_end(n, x);
        assert_eq!(result, right);
    }

    #[test]
    fn case_2() {
        let (n, x) = (2, 7);
        let right = 15;
        let result = Solution::min_end(n, x);
        assert_eq!(result, right);
    }
}
