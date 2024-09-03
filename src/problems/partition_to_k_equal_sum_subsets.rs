struct Solution;

// 698. 划分为k个相等的子集
// https://leetcode.cn/problems/partition-to-k-equal-sum-subsets

impl Solution {
    pub fn can_partition_k_subsets(mut nums: Vec<i32>, k: i32) -> bool {
        let sum: i32 = nums.iter().sum();
        if sum % k != 0 {
            return false;
        }

        nums.sort_unstable();

        let sum = sum / k;

        false
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn case_1() {
        let nums = [4, 3, 2, 3, 5, 2, 1];
        let k = 4;
        let right = true;
        let result = Solution::can_partition_k_subsets(nums.to_vec(), k);
        assert_eq!(result, right)
    }

    #[test]
    fn case_2() {
        let nums = [1, 2, 3, 4];
        let k = 3;
        let right = false;
        let result = Solution::can_partition_k_subsets(nums.to_vec(), k);
        assert_eq!(result, right)
    }
}
