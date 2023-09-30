struct Solution;

// https://leetcode.cn/problems/count-largest-group/

impl Solution {
    pub fn count_largest_group(n: i32) -> i32 {
        let mut sums = [0; 37];
        for i in 1..=n {
            sums[Solution::get_sum(i) as usize] += 1;
        }
        sums.into_iter()
            .filter(|&x| x == sums.into_iter().max().unwrap_or(0))
            .count() as i32
    }

    fn get_sum(mut num: i32) -> i32 {
        let mut sum = 0;
        while num != 0 {
            sum += num % 10;
            num /= 10;
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let n = 13;
        let right = 4;

        let result = Solution::count_largest_group(n);

        assert_eq!(result, right);
    }

    #[test]
    fn case_2() {
        let n = 2;
        let right = 2;

        let result = Solution::count_largest_group(n);

        assert_eq!(result, right);
    }
}
