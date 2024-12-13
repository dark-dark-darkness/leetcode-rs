struct Solution;

// 2717. 半有序排列
// https://leetcode.cn/problems/semi-ordered-permutation

impl Solution {
    pub fn semi_ordered_permutation(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        let x = nums.iter().position(|x| *x == 1).unwrap() as i32;
        let y = nums.iter().position(|x| *x == n).unwrap() as i32;
        if x < y {
            x + (n - y - 1)
        } else {
            x + (n - y - 1) - 1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn case_1() {
        let nums = [2, 1, 4, 3];
        let right = 2;
        let result = Solution::semi_ordered_permutation(nums.to_vec());
        assert_eq!(result, right)
    }

    #[test]
    fn case_2() {
        let nums = [2, 4, 1, 3];
        let right = 3;
        let result = Solution::semi_ordered_permutation(nums.to_vec());
        assert_eq!(result, right)
    }

    #[test]
    fn case_3() {
        let nums = [1, 3, 4, 2, 5];
        let right = 0;
        let result = Solution::semi_ordered_permutation(nums.to_vec());
        assert_eq!(result, right)
    }
}
