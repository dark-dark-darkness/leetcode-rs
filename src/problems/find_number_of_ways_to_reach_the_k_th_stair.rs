use std::collections::HashMap;

struct Solution;
// 3154. 到达第 K 级台阶的方案数
// https://leetcode.cn/problems/find-number-of-ways-to-reach-the-k-th-stair/description/
impl Solution {
    pub fn ways_to_reach_stair(k: i32) -> i32 {
        use std::collections::HashMap;
        dfs(1, 0, false, k, &mut HashMap::new())
    }
}

fn dfs(p: i32, jump: i32, backed: bool, k: i32, cache: &mut HashMap<(i32, i32, bool), i32>) -> i32 {
    if let Some(r) = cache.get(&(p, jump, backed)) {
        *r
    } else {
        let mut res = 0;

        if p == k {
            res += 1;
        }

        if !backed && p > 0 {
            let s = dfs(p - 1, jump, true, k, cache);
            cache.insert((p - 1, jump, true), s);
            res += s;
        }

        let t = p + 2_i32.pow(jump as u32);

        if t <= k + 1 {
            let s = dfs(t, jump + 1, false, k, cache);
            cache.insert((t, jump + 1, false), s);
            res += s;
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn case_1() {
        let k = 0;
        let right = 2;
        let result = Solution::ways_to_reach_stair(k);
        assert_eq!(result, right);
    }
}
