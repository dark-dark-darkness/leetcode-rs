use crate::Solution;

// https://leetcode.cn/problems/trapping-rain-water/

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let max = height.iter().max().unwrap_or(&0).clone();
        let mut sum = 0;
        for h in 1..=max {
            let l = {
                let mut index = 0;
                while index != height.len() {
                    if height[index] >= h {
                        break;
                    }
                    index += 1;
                }
                index
            };
            let r = {
                let mut index = height.len() - 1;
                while index != height.len() {
                    if height[index] >= h {
                        break;
                    }
                    index -= 1;
                }
                index
            };

            sum += height[l..r].iter().filter(|&&x| x < h).count();
        }
        sum as i32
    }
}

mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let nums = [0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
        let right = 6;

        let result = Solution::trap(nums.to_vec());

        assert_eq!(result, right);
    }

    #[test]
    fn case_2() {
        let nums = [4, 2, 0, 3, 2, 5];
        let right = 9;

        let result = Solution::trap(nums.to_vec());

        assert_eq!(result, right);
    }
}
