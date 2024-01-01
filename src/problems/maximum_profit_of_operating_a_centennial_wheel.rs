use std::collections::LinkedList;

struct Solution;

// 1599. 经营摩天轮的最大利润
// https://leetcode.cn/problems/maximum-profit-of-operating-a-centennial-wheel
impl Solution {
    pub fn min_operations_max_profit(
        customers: Vec<i32>,
        boarding_cost: i32,
        running_cost: i32,
    ) -> i32 {
        let mut result = [(0, 0), (-1, 0)];
        let mut waits = 0;

        let mut iter = customers.iter().peekable();
        loop {
            if let Some(c) = iter.next() {
                waits += c;
            }
            let count = if waits >= 4 { 4 } else { waits };

            result[0].0 += 1;
            result[0].1 += count * boarding_cost - running_cost;
            waits -= count;

            if result[0].1 > result[1].1 {
                result[1] = result[0];
            }
            if waits == 0 && iter.peek().is_none() {
                break;
            }
        }

        result[1].0
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn case_1() {
        let customers = [8, 3];
        let boarding_cost = 5;
        let running_cost = 6;

        let right = 3;
        let result =
            Solution::min_operations_max_profit(customers.to_vec(), boarding_cost, running_cost);
        assert_eq!(result, right)
    }

    #[test]
    fn case_2() {
        let customers = [10, 9, 6];
        let boarding_cost = 6;
        let running_cost = 4;

        let right = 7;
        let result =
            Solution::min_operations_max_profit(customers.to_vec(), boarding_cost, running_cost);
        assert_eq!(result, right)
    }

    #[test]
    fn case_3() {
        let customers = [3, 4, 0, 5, 1];
        let boarding_cost = 1;
        let running_cost = 92;

        let right = -1;
        let result =
            Solution::min_operations_max_profit(customers.to_vec(), boarding_cost, running_cost);
        assert_eq!(result, right)
    }

    #[test]
    fn case_4() {
        let customers = [10, 10, 6, 4, 7];
        let boarding_cost = 3;
        let running_cost = 8;

        let right = 9;
        let result =
            Solution::min_operations_max_profit(customers.to_vec(), boarding_cost, running_cost);
        assert_eq!(result, right)
    }

    #[test]
    fn case_5() {
        let customers = [
            0, 43, 37, 9, 23, 35, 18, 7, 45, 3, 8, 24, 1, 6, 37, 2, 38, 15, 1, 14, 39, 27, 4, 25,
            27, 33, 43, 8, 44, 30, 38, 40, 20, 5, 17, 27, 43, 11, 6, 2, 30, 49, 30, 25, 32, 3, 18,
            23, 45, 43, 30, 14, 41, 17, 42, 42, 44, 38, 18, 26, 32, 48, 37, 5, 37, 21, 2, 9, 48,
            48, 40, 45, 25, 30, 49, 41, 4, 48, 40, 29, 23, 17, 7, 5, 44, 23, 43, 9, 35, 26, 44, 3,
            26, 16, 31, 11, 9, 4, 28, 49, 43, 39, 9, 39, 37, 7, 6, 7, 16, 1, 30, 2, 4, 43, 23, 16,
            39, 5, 30, 23, 39, 29, 31, 26, 35, 15, 5, 11, 45, 44, 45, 43, 4, 24, 40, 7, 36, 10, 10,
            18, 6, 20, 13, 11, 20, 3, 32, 49, 34, 41, 13, 11, 3, 13, 0, 13, 44, 48, 43, 23, 12, 23,
            2,
        ];
        let boarding_cost = 43;
        let running_cost = 54;

        let right = 993;
        let result =
            Solution::min_operations_max_profit(customers.to_vec(), boarding_cost, running_cost);
        assert_eq!(result, right)
    }
}
