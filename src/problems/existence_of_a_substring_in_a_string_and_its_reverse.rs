struct Solution;

// 3083. 字符串及其反转中是否存在同一子字符串
// https://leetcode.cn/problems/existence-of-a-substring-in-a-string-and-its-reverse/
impl Solution {
    pub fn is_substring_present(s: String) -> bool {
        let s: Vec<u8> = s.bytes().collect();
        let rev_str: Vec<&u8> = s.iter().rev().collect();
        let subs = s.windows(2);
        for sub in subs {
            for i in 0..rev_str.len() - 1 {
                if *rev_str[i] == sub[0] && *rev_str[i + 1] == sub[1] {
                    return true;
                }
            }
        }
        false
    }
}
