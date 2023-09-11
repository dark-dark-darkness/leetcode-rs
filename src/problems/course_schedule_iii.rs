use crate::Solution;

// https://leetcode.cn/problems/course-schedule-iii/

impl Solution {
    pub fn schedule_course(mut courses: Vec<Vec<i32>>) -> i32 {
        use std::collections::BinaryHeap;
        courses.sort_unstable_by(|a, b| a[1].cmp(&b[1]));
        let mut sum = 0;
        let mut heap = BinaryHeap::new();
        for c in courses {
            heap.push(c[0]);
            sum += c[0];
            if sum > c[1] {
                sum -= heap.pop().unwrap();
            }
        }
        heap.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn case_1() {
        let courses = [[100, 200], [200, 1300], [1000, 1250], [2000, 3200]];

        let right = 3;

        let result = Solution::schedule_course(courses.map(|x| x.into()).into());

        assert_eq!(result, right)
    }

    #[test]
    fn case_2() {
        let courses = [[1, 2]];

        let right = 1;

        let result = Solution::schedule_course(courses.map(|x| x.into()).into());

        assert_eq!(result, right)
    }

    #[test]
    fn case_3() {
        let courses = [[3, 2], [4, 3]];

        let right = 0;

        let result = Solution::schedule_course(courses.map(|x| x.into()).into());

        assert_eq!(result, right)
    }

    #[test]
    fn case_4() {
        let courses = [[1, 2], [2, 3]];

        let right = 2;

        let result = Solution::schedule_course(courses.map(|x| x.into()).into());

        assert_eq!(result, right)
    }

    #[test]
    fn case_5() {
        let courses = [[5, 5], [4, 6], [2, 6]];

        let right = 2;

        let result = Solution::schedule_course(courses.map(|x| x.into()).into());

        assert_eq!(result, right)
    }
}
