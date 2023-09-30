struct Solution;

// https://leetcode.cn/problems/median-of-two-sorted-arrays/

impl Solution {
    pub fn find_median_sorted_arrays(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> f64 {
        nums1.append(&mut nums2);
        nums1.sort_unstable();

        if nums1.len() & 1 == 1 {
            nums1[nums1.len() / 2] as f64
        } else {
            (nums1[nums1.len() / 2] as f64 + nums1[nums1.len() / 2 - 1] as f64) / 2.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let input1 = [1, 3];
        let input2 = [2];

        let right = 2.0;

        let result = Solution::find_median_sorted_arrays(input1.to_vec(), input2.to_vec());

        assert_eq!(result, right)
    }

    #[test]
    fn case_2() {
        let input1 = [1, 2];
        let input2 = [3, 4];

        let right = 2.50000;

        let result = Solution::find_median_sorted_arrays(input1.to_vec(), input2.to_vec());

        assert_eq!(result, right)
    }
}
