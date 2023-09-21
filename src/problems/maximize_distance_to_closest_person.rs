struct Solution;

// https://leetcode.cn/problems/maximize-distance-to-closest-person/

impl Solution {
    pub fn max_dist_to_closest(seats: Vec<i32>) -> i32 {
        let len = seats.len();
        let mut seats = seats
            .iter()
            .enumerate()
            .filter(|(_, &v)| v == 1)
            .map(|(i, _)| i)
            .peekable();

        let mut max_d = if let Some(&v @ 1..) = seats.peek() {
            v
        } else {
            0
        };

        while let Some(l) = seats.next() {
            if let Some(r) = seats.peek() {
                let d = (r - l) / 2;
                max_d = max_d.max(d);
            } else if l != len {
                max_d = max_d.max(len - l - 1);
            }
        }

        max_d as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let seats = [1, 0, 0, 0, 1, 0, 1]; // 0 4 6
        let right = 2;

        let result = Solution::max_dist_to_closest(seats.to_vec());

        assert_eq!(result, right);
    }

    #[test]
    fn case_2() {
        let seats = [1, 0, 0, 0];
        let right = 3;

        let result = Solution::max_dist_to_closest(seats.to_vec());

        assert_eq!(result, right);
    }

    #[test]
    fn case_3() {
        let seats = [0, 1];
        let right = 1;

        let result = Solution::max_dist_to_closest(seats.to_vec());

        assert_eq!(result, right);
    }
}
