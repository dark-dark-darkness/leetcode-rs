use std::collections::HashSet;

struct Solution;
// 26. 删除有序数组中的重复项 II
// https://leetcode.cn/problems/remove-duplicates-from-sorted-array
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let len = nums.len();
        if len < 2 {
            return len as i32;
        }
        let (mut fast, mut slow) = (2, 2);

        while fast < len {
            if nums[slow - 2] != nums[fast] {
                nums[slow] = nums[fast];
                slow += 1;
            }
            fast += 1;
        }

        slow as i32
    }
}
