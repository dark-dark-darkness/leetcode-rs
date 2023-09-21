struct Solution;

// https://leetcode.cn/problems/number-of-ways-to-buy-pens-and-pencils/

impl Solution {
    pub fn ways_to_buy_pens_pencils(total: i32, cost1: i32, cost2: i32) -> i64 {
        if cost1 < cost2 {
            return Self::ways_to_buy_pens_pencils(total, cost2, cost1);
        }
        let (total, cost1, cost2) = (total as i64, cost1 as i64, cost2 as i64);

        let mut result = 0;

        let mut n1 = 0;

        while n1 * cost1 <= total {
            result += (total - cost1 * n1) / cost2 + 1;

            n1 += 1;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let total = 20;
        let cost1 = 10;
        let cost2 = 5;

        let right = 9;

        let result = Solution::ways_to_buy_pens_pencils(total, cost1, cost2);

        assert_eq!(result, right)
    }

    #[test]
    fn case_2() {
        let total = 5;
        let cost1 = 10;
        let cost2 = 10;

        let right = 1;

        let result = Solution::ways_to_buy_pens_pencils(total, cost1, cost2);

        assert_eq!(result, right)
    }
}
