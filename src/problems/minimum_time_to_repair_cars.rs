struct Solution;

// https://leetcode.cn/problems/minimum-time-to-repair-cars

impl Solution {
    pub fn repair_cars(mut ranks: Vec<i32>, cars: i32) -> i64 {
        ranks.sort_unstable();
        let cars = cars as i64;
        let mut l = 1;
        let mut r = i64::MAX;
        while l < r {
            let mid = ((r - l) >> 1) + l;
            let mut top = false;
            let mut t = 0;
            for &v in &ranks {
                t += ((mid / (v as i64)) as f64).sqrt() as i64;
                if t >= cars {
                    top = true;
                    break;
                }
            }
            if top {
                r = mid;
            } else {
                l = mid + 1;
            }
        }
        l
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let ranks = [4, 2, 3, 1];
        let cars = 10;

        let right = 16;

        let result = Solution::repair_cars(ranks.into(), cars);

        assert_eq!(result, right);
    }

    #[test]
    fn case_2() {
        let ranks = [5, 1, 8];
        let cars = 6;

        let right = 16;

        let result = Solution::repair_cars(ranks.into(), cars);

        assert_eq!(result, right);
    }
}
