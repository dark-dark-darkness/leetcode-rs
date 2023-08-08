use crate::Solution;

// https://leetcode.cn/problems/3sum/

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn case1() {
        assert_eq!(
            Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]),
            vec![vec![-1, 0, 1], vec![-1, -1, 2]]
        )
    }

    #[test]
    fn case2() {
        assert_eq!(Solution::three_sum(vec![0, 1, 1]), Vec::<Vec<i32>>::new())
    }

    #[test]
    fn case3() {
        assert_eq!(Solution::three_sum(vec![0, 0, 0]), vec![vec![0, 0, 0]])
    }

    #[test]
    fn case4() {
        assert_eq!(Solution::three_sum(vec![-1, 0, 1, 0]), vec![vec![-1, 0, 1]])
    }

    #[test]
    fn case5() {
        assert_eq!(
            Solution::three_sum(vec![3, 0, -2, -1, 1, 2]),
            vec![vec![-2, -1, 3], vec![-2, 0, 2], vec![-1, 0, 1]]
        )
    }
}
