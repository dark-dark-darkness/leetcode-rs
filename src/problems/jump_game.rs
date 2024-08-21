struct Solution;
// 121. 买卖股票的最佳时机
// https://leetcode.cn/problems/best-time-to-buy-and-sell-stock
impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut p = 0;
        loop {
            let x = nums[dbg!(p..(p + nums[p] as usize) + 1)]
                .iter()
                .enumerate()
                .map(|(i, o)| i - p + *o as usize)
                .inspect(|x| {
                    dbg!(x);
                })
                .max()
                .unwrap_or(0);
            if x == 0 {
                return false;
            }
            p += x;

            if p > nums.len() {
                return true;
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn case_1() {
        let nums = vec![2, 3, 1, 1, 4];
        let right = true;
        let result = Solution::can_jump(nums);
        assert_eq!(result, right)
    }

    #[test]
    fn case_2() {
        let nums = vec![3, 2, 1, 0, 4];
        let right = false;
        let result = Solution::can_jump(nums);
        assert_eq!(result, right)
    }
}
