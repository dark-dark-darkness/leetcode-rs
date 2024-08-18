struct Solution;

// 600. 不含连续1的非负整数
// https://leetcode.cn/problems/non-negative-integers-without-consecutive-ones

impl Solution {
    pub fn find_integers(n: i32) -> i32 {
        let n = n + 1;
        let b: Vec<_> = format!("{:b}", n).chars().rev().collect();
        let n = b.len();
        let dp: Vec<_> = (0..n)
            .scan((1, 0), |s, _| {
                *s = (s.0 + s.1, s.0);
                Some(*s)
            })
            .collect();
        let mut ans = 0;
        for i in (0..n).rev() {
            if b[i] == '1' {
                ans += dp[i].0;
            }
            if i < b.len() - 1 && b[i] == '1' && b[i + 1] == '1' {
                break;
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn case_1() {
        let n = 5;
        let right = 5;
        let result = Solution::find_integers(n);
        assert_eq!(result, right);
    }

    #[test]
    fn case_2() {
        let n = 1;
        let right = 2;
        let result = Solution::find_integers(n);
        assert_eq!(result, right);
    }

    #[test]
    fn case_3() {
        let n = 2;
        let right = 3;
        let result = Solution::find_integers(n);
        assert_eq!(result, right);
    }

    #[test]
    fn case_4() {
        let n = 12;
        let right = 8;
        let result = Solution::find_integers(n);
        assert_eq!(result, right);
    }
}
