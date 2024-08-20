struct Solution;

// 551. 学生出勤记录 I
// https://leetcode.cn/problems/student-attendance-record-i/

impl Solution {
    pub fn check_record(s: String) -> bool {
        let mut ac = 0;
        let mut lc = 0;
        for c in s.chars() {
            if c == 'A' {
                ac += 1;
                lc = 0;
            } else if c == 'L' {
                lc += 1;
            } else {
                lc = 0;
            }

            if ac >= 2 {
                return false;
            }

            if lc >= 3 {
                return false;
            }
        }

        true
    }
}
