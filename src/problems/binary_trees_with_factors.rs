use crate::Solution;

// https://leetcode.cn/problems/binary-trees-with-factors

const MOD: u64 = 1_000_000_007;

impl Solution {
    pub fn num_factored_binary_trees(mut arr: Vec<i32>) -> i32 {
        arr.sort_unstable();

        let n = arr.len();
        let mut dp = vec![1; n];
        let mut mp = std::collections::HashMap::new();
        let mut ans = 0;

        for i in 0..n {
            mp.insert(arr[i], i);

            for j in 0..i {
                if arr[i] % arr[j] == 0 {
                    if let Some(&k) = mp.get(&(arr[i] / arr[j])) {
                        dp[i] += dp[j] * dp[k];
                    }
                }
            }

            ans = (ans + dp[i]) % MOD;
        }

        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn case_1() {
        let arr = [2, 4];
        let right = 3;

        let result = Solution::num_factored_binary_trees(arr.to_vec());

        assert_eq!(result, right);
    }

    #[test]
    fn case_2() {
        let arr = [2, 4, 5, 10];
        let right = 7;

        let result = Solution::num_factored_binary_trees(arr.to_vec());

        assert_eq!(result, right);
    }
}
