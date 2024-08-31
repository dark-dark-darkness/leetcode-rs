struct Solution;
// 189. 轮转数组
// https://leetcode.cn/problems/rotate-array

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        if nums.len() == 1 {
            return;
        }
        let (head, tail) = nums.split_at(nums.len() - (k as usize % nums.len()));
        *nums = tail.iter().chain(head).copied().collect();
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn case_1() {
        let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
        let k = 3;
        let right = vec![5, 6, 7, 1, 2, 3, 4];
        Solution::rotate(&mut nums, k);
        assert_eq!(nums, right)
    }
}
