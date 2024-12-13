struct Solution;
// 887. 鸡蛋掉落
// https://leetcode.cn/problems/super-egg-drop
use std::collections::HashMap;

impl Solution {
    pub fn super_egg_drop(k: i32, n: i32) -> i32 {
        Self::dp(k, n, &mut HashMap::new())
    }

    fn dp(k: i32, n: i32, memo: &mut HashMap<(i32, i32), i32>) -> i32 {
        if let Some(value) = memo.get(&(k, n)) {
            return *value;
        }

        let mut ans = 0;

        if n == 0 {
            ans = 0;
        } else if k == 1 {
            ans = n;
        } else {
            let mut lo = 1;
            let mut hi = n;
            while lo + 1 < hi {
                let x = lo + hi;
                let t1 = Self::dp(k - 1, x - 1, memo);
                let t2 = Self::dp(k, n - x, memo);
                match i32::cmp(&t1, &t2) {
                    std::cmp::Ordering::Less => lo = x,
                    std::cmp::Ordering::Greater => hi = x,
                    std::cmp::Ordering::Equal => {
                        lo = x;
                        hi = x;
                    }
                }
            }
            ans = 1
                + (Self::dp(k - 1, lo - 1, memo).max(Self::dp(k, n - lo, memo)))
                    .min(Self::dp(k - 1, hi - 1, memo).max(Self::dp(k, n - hi, memo)))
        }

        memo.insert((k, n), ans);
        ans
    }
}