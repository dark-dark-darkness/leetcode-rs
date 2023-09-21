struct Solution;

// https://leetcode.cn/problems/reverse-string/

impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        let len = s.len();
        for li in 0..(len / 2) {
            s.swap(len - li - 1, li)
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn case_1() {
        let mut input = vec!['h', 'e', 'l', 'l', 'o'];
        let right = vec!['o', 'l', 'l', 'e', 'h'];

        Solution::reverse_string(&mut input);

        assert_eq!(input, right)
    }

    #[test]
    fn case_2() {
        let mut input = vec!['H', 'a', 'n', 'n', 'a', 'h'];
        let right = vec!['h', 'a', 'n', 'n', 'a', 'H'];

        Solution::reverse_string(&mut input);

        assert_eq!(input, right)
    }
}
