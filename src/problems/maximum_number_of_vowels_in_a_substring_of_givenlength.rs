struct Solution;
// 1456. 定长子串中元音的最大数目
// https://leetcode.cn/problems/maximum-number-of-vowels-in-a-substring-of-given-length/

impl Solution {
    pub fn max_vowels(s: String, k: i32) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let k = k as usize;
        let len = chars.len();
        let mut vowel_count = chars[0..k].iter().map(|&x| Self::is_vowel(x)).sum::<i32>();
        let mut ans = vowel_count;
        for i in 1..len - k + 1 {
            vowel_count += Self::is_vowel(chars[i + k - 1]) - Self::is_vowel(chars[i - 1]);
            ans = ans.max(vowel_count);
        }
        ans
    }

    fn is_vowel(c: char) -> i32 {
        if c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u' {
            1
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn case_1() {
        let (s, k) = ("abciiidef", 3);
        let right = 3;
        let result = Solution::max_vowels(s.to_string(), k);
        assert_eq!(result, right)
    }

    #[test]
    fn case_2() {
        let (s, k) = ("aeiou", 2);
        let right = 2;
        let result = Solution::max_vowels(s.to_string(), k);
        assert_eq!(result, right)
    }

    #[test]
    fn case_3() {
        let (s, k) = ("leetcode", 3);
        let right = 2;
        let result = Solution::max_vowels(s.to_string(), k);
        assert_eq!(result, right)
    }

    #[test]
    fn case_4() {
        let (s, k) = ("rhythms", 4);
        let right = 0;
        let result = Solution::max_vowels(s.to_string(), k);
        assert_eq!(result, right)
    }

    #[test]
    fn case_5() {
        let (s, k) = ("tryhard", 4);
        let right = 1;
        let result = Solution::max_vowels(s.to_string(), k);
        assert_eq!(result, right)
    }

    #[test]
    fn case_6() {
        let (s, k) = ("weallloveyou", 7);
        let right = 4;
        let result = Solution::max_vowels(s.to_string(), k);
        assert_eq!(result, right)
    }
}
