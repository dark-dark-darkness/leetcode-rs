struct Solution;
// 3134. 找出唯一性数组的中位数
// https://leetcode.cn/problems/find-the-losers-of-the-circular-game/
impl Solution {
    pub fn median_of_uniqueness_array(mut nums: Vec<i32>) -> i32 {
        0
    }

    fn distinct(nums: &[i32], start: usize, end: usize) -> i32 {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn case_1() {
        let nums = [1, 2, 3];
        let right = 1;

        let result = Solution::median_of_uniqueness_array(nums.to_vec());

        assert_eq!(result, right)
    }

    #[test]
    fn case_2() {
        let nums = [3, 4, 3, 4, 5];
        let right = 1;

        let result = Solution::median_of_uniqueness_array(nums.to_vec());

        assert_eq!(result, right)
    }

    #[test]
    fn case_3() {
        let nums = [4, 3, 5, 4];
        let right = 1;

        let result = Solution::median_of_uniqueness_array(nums.to_vec());

        assert_eq!(result, right)
    }
}
