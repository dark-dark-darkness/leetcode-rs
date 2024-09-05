struct Solution;

// 3174. 清除数字
// https://leetcode.cn/problems/clear-digits/description/

impl Solution {
    pub fn clear_digits(s: String) -> String {
        let mut i = 0;
        let mut chars: Vec<char> = s.chars().collect();

        loop {
            if i + 1 >= chars.len() {
                break;
            }
            let c = chars[i + 1];
            if c.is_ascii_digit() {
                chars.remove(i);
                chars.remove(i);
                i = i.saturating_sub(1);
            } else {
                i += 1;
            }
        }

        chars.iter().collect::<String>()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let s = "abc";
        let right = "abc";
        let result = Solution::clear_digits(s.to_string());
        assert_eq!(result, right);
    }

    #[test]
    fn case_2() {
        let s = "cb34";
        let right = "";
        let result = Solution::clear_digits(s.to_string());
        assert_eq!(result, right);
    }
}
