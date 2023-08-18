use crate::Solution;

// https://leetcode.cn/problems/maximum-subarray/

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        todo!("{:?}", nums)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let input = [-2, 1, -3, 4, -1, 2, 1, -5, 4];
        let right = 6;

        let result = Solution::max_sub_array(input.to_vec());

        assert_eq!(result, right);
    }

    #[test]
    fn case_2() {
        let input = [1];
        let right = 1;

        let result = Solution::max_sub_array(input.to_vec());

        assert_eq!(result, right);
    }

    #[test]
    fn case_3() {
        let input = [5, 4, -1, 7, 8];
        let right = 23;

        let result = Solution::max_sub_array(input.to_vec());

        assert_eq!(result, right);
    }

    #[test]
    fn case_4() {
        let input = [-2, 1];
        let right = 1;

        let result = Solution::max_sub_array(input.to_vec());

        assert_eq!(result, right);
    }

    #[test]
    fn case_5() {
        let input = [-9, 9, 7, -8, -3, 2, -9, 1, 7, -2, -9, 7, -9, 5, 5, -3, 9, 0];
        let right = 16;

        let result = Solution::max_sub_array(input.to_vec());

        assert_eq!(result, right);
    }

    #[test]
    fn case_6() {
        let input = [
            -84, -87, -78, -16, -94, -36, -87, -93, -50, -22, -63, -28, -91, -60, -64, -27, -41,
            -27, -73, -37, -12, -69, -68, -30, -83, -31, -63, -24, -68, -36, -30, -3, -23, -59,
            -70, -68, -94, -57, -12, -43, -30, -74, -22, -20, -85, -38, -99, -25, -16, -71, -14,
            -27, -92, -81, -57, -74, -63, -71, -97, -82, -6, -26, -85, -28, -37, -6, -47, -30, -14,
            -58, -25, -96, -83, -46, -15, -68, -35, -65, -44, -51, -88, -9, -77, -79, -89, -85, -4,
            -52, -55, -100, -33, -61, -77, -69, -40, -13, -27, -87, -95, -40,
        ];
        let right = -3;

        let result = Solution::max_sub_array(input.to_vec());

        assert_eq!(result, right);
    }

    #[test]
    fn case_7() {
        let input = [-1];
        let right = -1;

        let result = Solution::max_sub_array(input.to_vec());

        assert_eq!(result, right);
    }

    #[test]
    fn case_8() {
        let input = [1, 2, -1, -2, 2, 1, -2, 1, 4, -5, 4];
        let right = 6;

        let result = Solution::max_sub_array(input.to_vec());

        assert_eq!(result, right);
    }
}
