struct Solution;

// 2390. 从字符串中移除星号
// https://leetcode.cn/problems/removing-stars-from-a-string

impl Solution {
    pub fn remove_stars(s: String) -> String {
        use std::collections::VecDeque;
        let chars: Vec<char> = s.chars().collect();

        let mut stack: VecDeque<char> = VecDeque::with_capacity(chars.len());

        for c in chars {
            if c == '*' {
                stack.pop_back();
            } else {
                stack.push_back(c)
            }
        }

        stack.iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn case_1() {
        let s = "leet**cod*e";
        let right = "lecoe";
        let result = Solution::remove_stars(s.to_string());
        assert_eq!(result.to_string(), right.to_string());
    }

    #[test]
    fn case_2() {
        let s = "erase*****";
        let right = "";
        let result = Solution::remove_stars(s.to_string());
        assert_eq!(result.to_string(), right.to_string());
    }
}
