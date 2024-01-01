struct Solution;
// 88. 合并两个有序数组
// https://leetcode.cn/problems/merge-sorted-array
impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &Vec<i32>, n: i32) {
        if n == 0 {
            return;
        }

        let (mut p1, mut p2) = (m as usize, n as usize);
        let mut tail = (m + n) as usize;

        while p2 > 0 {
            if p1 == 0 || nums2[p2 - 1] > nums1[p1 - 1] {
                nums1[tail - 1] = nums2[p2 - 1];
                p2 -= 1;
            } else {
                nums1[tail - 1] = nums1[p1 - 1];
                p1 -= 1;
            }
            tail -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn case_1() {
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        let m = 3;
        let nums2 = [2, 5, 6];
        let n = 3;
        Solution::merge(&mut nums1, m, &nums2.to_vec(), n);

        let right = vec![1, 2, 2, 3, 5, 6];

        assert_eq!(nums1, right)
    }

    #[test]
    fn case_2() {
        let mut nums1 = vec![4, 5, 6, 0, 0, 0];
        let m = 3;
        let nums2 = [1, 2, 3];
        let n = 3;
        Solution::merge(&mut nums1, m, &nums2.to_vec(), n);

        let right = vec![1, 2, 3, 4, 5, 6];

        assert_eq!(nums1, right)
    }

    #[test]
    fn case_3() {
        let mut nums1 = vec![0, 0, 3, 0, 0, 0, 0, 0, 0];
        let m = 3;
        let nums2 = [-1, 1, 1, 1, 2, 3];
        let n = 6;
        Solution::merge(&mut nums1, m, &nums2.to_vec(), n);

        let right = vec![-1, 0, 0, 1, 1, 1, 2, 3, 3];

        assert_eq!(nums1, right)
    }
}
