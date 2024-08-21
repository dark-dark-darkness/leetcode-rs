struct Solution;
// 3007. 价值和小于等于 K 的最大数字
// https://leetcode.cn/problems/maximum-number-that-sum-of-the-prices-is-less-than-or-equal-to-k/

impl Solution {
    pub fn find_maximum_number(k: i64, x: i32) -> i64 {
        let (mut l, mut r) = (1i64, (k + 1) << x);
        while l < r {
            let m = (l + r + 1) / 2;
            if Self::accumulated_price(x, m) > k {
                r = m - 1;
            } else {
                l = m;
            }
        }
        l
    }

    fn accumulated_bit_price(x: i32, num: i64) -> i64 {
        let period = 1i64 << x;
        let mut res = period / 2 * (num / period);
        if num % period >= period / 2 {
            res += num % period - (period / 2 - 1);
        }
        res
    }

    fn accumulated_price(x: i32, num: i64) -> i64 {
        let mut res = 0i64;
        let length = 64 - num.leading_zeros();
        for i in (x..=length as i32).step_by(x as usize) {
            res += Self::accumulated_bit_price(i, num);
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn case_1() {
        let (k, x) = (9, 1);
        let right = 6;
        let result = Solution::find_maximum_number(k, x);
        assert_eq!(result, right);
    }

    #[test]
    fn case_2() {
        let (k, x) = (7, 2);
        let right = 9;
        let result = Solution::find_maximum_number(k, x);
        assert_eq!(result, right);
    }
}
