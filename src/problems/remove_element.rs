struct Solution;

// 27. 移除元素
// https://leetcode.cn/problems/remove-element

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut slow = 0;
        let mut fast = 0;
        while fast < nums.len() {
            if nums[slow] == val && nums[fast] != val {
                nums.swap(fast, slow);
            } else if nums[slow] == val && nums[fast] == val {
                fast += 1;
            } else {
                if fast == slow {
                    fast = slow + 1;
                }
                slow += 1;
            }
        }
        slow as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn case_1() {
        let mut nums = [3, 2, 2, 3];
        let val = 3;
        let right_i = 2;
        let right_nums = [2, 2];

        let result = Solution::remove_element(&mut nums.to_vec(), val);

        dbg!(&nums);

        assert_eq!(result, right_i);

        let nums = &mut nums[..(result as usize)];

        nums.sort();

        assert_eq!(nums, right_nums);
    }
}
