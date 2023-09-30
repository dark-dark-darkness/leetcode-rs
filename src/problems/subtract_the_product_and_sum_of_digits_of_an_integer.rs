struct Solution;

// https://leetcode.cn/problems/subtract-the-product-and-sum-of-digits-of-an-integer/

impl Solution {
    pub fn subtract_product_and_sum(mut n: i32) -> i32 {
        let mut sum = 0;
        let mut product = 1;

        while n != 0 {
            let x = n % 10;
            sum += x;
            product *= x;
            n /= 10;
        }

        product - sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let input = 234;
        let right = 15;

        let result = Solution::subtract_product_and_sum(input);

        assert_eq!(result, right);
    }
}
