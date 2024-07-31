struct Solution;

// https://leetcode.cn/problems/delete-and-earn
// 740. 删除并获得点数

impl Solution {
    pub fn delete_and_earn(nums: Vec<i32>) -> i32 {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn case_1() {
        let nums = [3, 4, 2];
        let right = 6;
        let result = Solution::delete_and_earn(nums.to_vec());
        assert_eq!(result, right)
    }

    #[test]
    fn case_2() {
        let nums = [2, 2, 3, 3, 3, 4];
        let right = 9;
        let result = Solution::delete_and_earn(nums.to_vec());
        assert_eq!(result, right)
    }
}
