struct Solution;

// 1450. 在既定时间做作业的学生人数
// https://leetcode.cn/problems/number-of-ways-of-cutting-a-pizza/
impl Solution {
    pub fn busy_student(start_time: Vec<i32>, end_time: Vec<i32>, query_time: i32) -> i32 {
        let mut cnt = 0;
        let len = start_time.len();
        for i in 0..len {
            if start_time[i] <= query_time && end_time[i] >= query_time {
                cnt += 1;
            }
        }
        cnt
    }
}
