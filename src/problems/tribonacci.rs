use std::collections::HashMap;

struct Solution;

// 1137. 第 N 个泰波那契数
// https://leetcode.cn/problems/n-th-tribonacci-number

impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        tribonacci_core(n, &mut HashMap::new())
    }
}

fn tribonacci_core(n: i32, cache: &mut HashMap<i32, i32>) -> i32 {
    if let Some(res) = cache.get(&n) {
        res.clone()
    } else {
        let result = match n {
            0 => 0,
            1 => 1,
            2 => 1,
            n => {
                tribonacci_core(n - 3, cache)
                    + tribonacci_core(n - 2, cache)
                    + tribonacci_core(n - 1, cache)
            }
        };
        cache.insert(n, result);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn case_1() {
        let n = 25;
        let right = 1389537;

        let result = Solution::tribonacci(n);

        assert_eq!(result, right)
    }
    #[test]
    fn case_2() {
        let n = 29;
        let right = 15902591;

        let result = Solution::tribonacci(n);

        assert_eq!(result, right)
    }
}
