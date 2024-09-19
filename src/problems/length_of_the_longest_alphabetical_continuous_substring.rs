struct Solution;
// 2414. 最长的字母序连续子字符串的长度
// https://leetcode.cn/problems/length-of-the-longest-alphabetical-continuous-substring

impl Solution {
    pub fn longest_continuous_substring(s: String) -> i32 {
        let mut ans = -1;
        let mut pre = None;
        let mut l = 1;

        for cur in s.bytes() {
            if l == 26 {
                return 26;
            }
            if let Some(pre_b) = pre {
                // dbg!(pre_b, cur);
                if pre_b + 1 == cur {
                    l += 1;
                } else {
                    ans = ans.max(l);
                    l = 1;
                }
            }
            pre = Some(cur);
        }

        ans.max(l)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let s = "abacaba";
        let right = 2;
        let result = Solution::longest_continuous_substring(s.to_string());
        assert_eq!(result, right);
    }

    #[test]
    fn case_2() {
        let s = "abcde";
        let right = 5;
        let result = Solution::longest_continuous_substring(s.to_string());
        assert_eq!(result, right);
    }

    #[test]
    fn case_3() {
        let s = "pcfftiovoygwwncfgews";
        let right = 2;
        let result = Solution::longest_continuous_substring(s.to_string());
        assert_eq!(result, right);
    }
}
