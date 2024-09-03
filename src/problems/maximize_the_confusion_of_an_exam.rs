struct Solution;

// 2024. 考试的最大困扰度
// https://leetcode.cn/problems/maximize-the-confusion-of-an-exam
impl Solution {
    pub fn max_consecutive_answers(answer_key: String, k: i32) -> i32 {
        let mut l = 0;
        let mut r = 1;
        let keys: Vec<char> = answer_key.chars().collect();
        let mut tc = if keys[0] == 'T' { 1 } else { 0 };
        let mut fc = if keys[0] == 'T' { 0 } else { 1 };

        while r < keys.len() {
            if tc <= k || fc <= k {
                r += 1;
                if keys[r - 1] == 'T' {
                    tc += 1;
                } else {
                    fc += 1;
                }
            } else {
                l += 1;
                r += 1;
                println!("{} {}", l, r);
                if keys[l - 1] == 'T' {
                    tc -= 1;
                } else {
                    fc -= 1;
                }
                if keys[r - 1] == 'T' {
                    tc += 1;
                } else {
                    fc += 1;
                }
            }
        }

        (r - l) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn case_1() {
        let answer_key = "TTFF";
        let k = 2;
        let right = 4;
        let result = Solution::max_consecutive_answers(answer_key.to_string(), k);
        assert_eq!(result, right);
    }

    #[test]
    fn case_2() {
        let answer_key = "TTFF";
        let k = 2;
        let right = 4;
        let result = Solution::max_consecutive_answers(answer_key.to_string(), k);
        assert_eq!(result, right);
    }

    #[test]
    fn case_3() {
        let answer_key = "TTFTTFTT";
        let k = 1;
        let right = 5;
        let result = Solution::max_consecutive_answers(answer_key.to_string(), k);
        assert_eq!(result, right);
    }
}
