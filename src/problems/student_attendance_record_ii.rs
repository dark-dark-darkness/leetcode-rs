struct Solution;
// 552. 学生出勤记录 II
// https://leetcode.cn/problems/student-attendance-record-ii

const MOD: i32 = 1000000007;

impl Solution {
    pub fn check_record(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![vec![vec![0; 3]; 2]; n + 1];
        dp[0][0][0] = 1;

        for i in 1..dp.len() + 1 {
            for j in 0..2 {
                for k in 0..3 {
                    dp[i][j][0] = (dp[i][j][0] + dp[i - 1][j][k]) % MOD;
                }
            }
            for k in 0..3 {
                dp[i][1][0] = (dp[i][1][0] + dp[i - 1][0][k]) % MOD;
            }
            for j in 0..2 {
                for k in 1..3 {
                    dp[i][j][k] = (dp[i][j][k] + dp[i - 1][j][k - 1]) % MOD;
                }
            }
        }

        dbg!(&dp);

        dp[n].iter().flatten().sum::<i32>() % MOD
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn case_1() {
        let n = 2;
        let right = 8;
        let result = Solution::check_record(n);
        assert_eq!(result, right);
    }

    #[test]
    fn case_2() {
        let n = 1;
        let right = 3;
        let result = Solution::check_record(n);
        assert_eq!(result, right);
    }

    #[test]
    fn case_3() {
        let n = 10101;
        let right = 183236316;
        let result = Solution::check_record(n);
        assert_eq!(result, right);
    }
}
