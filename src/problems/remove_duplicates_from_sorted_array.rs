struct Solution;
// 26. 删除有序数组中的重复项
// https://leetcode.cn/problems/remove-duplicates-from-sorted-array

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let len = nums.len();
        let (mut fast, mut slow) = (1, 1);
        while fast < len {
            if nums[fast] != nums[fast - 1] {
                nums[slow] = nums[fast];
                slow += 1;
            }
            fast += 1;
        }

        slow as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn case_1() {
        let mut nums = vec![1, 1, 2];
        let right_i = 2;
        let right_nums = [1, 2];

        let result = Solution::remove_duplicates(&mut nums);

        dbg!(&nums);

        assert_eq!(result, right_i);

        let nums = &mut nums[..(result as usize)];

        nums.sort();

        assert_eq!(nums, right_nums);
    }

    #[test]
    fn case_2() {
        let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        let right_i = 5;
        let right_nums = [0, 1, 2, 3, 4];

        let result = Solution::remove_duplicates(&mut nums);

        dbg!(&nums);

        assert_eq!(result, right_i);

        let nums = &mut nums[..(result as usize)];

        nums.sort();

        assert_eq!(nums, right_nums);
    }
}
