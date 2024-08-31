struct Solution;
// 169. 多数元素
// https://leetcode.cn/problems/majority-element

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut cnt = 0;
        let mut candidate = None;
        for num in nums {
            if cnt == 0 {
                candidate = Some(num);
            }

            cnt += match candidate {
                Some(n) if n == num => 1,
                _ => -1,
            }
        }

        candidate.unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn case_1() {
        let nums = [6, 5, 5];
        let right = 5;
        let result = Solution::majority_element(nums.to_vec());
        assert_eq!(result, right)
    }
}
