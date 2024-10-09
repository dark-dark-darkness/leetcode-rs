struct Solution;

// 3171. 找到按位或最接近 K 的子数组
// https://leetcode.cn/problems/find-subarray-with-bitwise-or-closest-to-k/
impl Solution {
    pub fn minimum_difference(mut nums: Vec<i32>, k: i32) -> i32 {
        let mut ans = i32::MAX;
        let mut left = 0;
        let mut bottom = 0;
        let mut right_or = 0;
        for right in 0..nums.len() {
            right_or |= nums[right];
            while left <= right && (nums[left] | right_or) > k {
                ans = ans.min((nums[left] | right_or) - k);
                if bottom <= left {
                    // 重新构建一个栈
                    // 由于 left 即将移出窗口，只需计算到 left+1
                    for i in (left + 1..right).rev() {
                        nums[i] |= nums[i + 1];
                    }
                    bottom = right;
                    right_or = 0;
                }
                left += 1;
            }
            if left <= right {
                ans = ans.min(k - (nums[left] | right_or));
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn case_1() {
        let nums = [1, 2, 4, 5]; // [1,3,7,7]
        let k = 3;
        let right = 0;
        let result = Solution::minimum_difference(nums.to_vec(), k);
        assert_eq!(result, right)
    }

    #[test]
    fn case_2() {
        let nums = [1, 3, 1, 3]; // [1,3,3,3]
        let k = 2;
        let right = 1;
        let result = Solution::minimum_difference(nums.to_vec(), k);
        assert_eq!(result, right)
    }

    #[test]
    fn case_3() {
        let nums = [1];
        let k = 10;
        let right = 9;
        let result = Solution::minimum_difference(nums.to_vec(), k);
        assert_eq!(result, right)
    }
}
