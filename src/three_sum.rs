use std::collections::HashSet;

use crate::Solution;

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();
        let mut result = HashSet::new();
        let mut set = HashSet::new();

        for i in 0..nums.len() {
            let mut flag: bool = false;

            for j in (i + 1)..nums.len() {
                if nums[i] > 0 && nums[j] > 0 {
                    flag = true;
                    break;
                }
                set.insert((i, j));
            }

            if flag {
                break;
            }
        }

        for (i, j) in set {
            if let Ok(k) = nums.binary_search(&(-1 * (nums[i] + nums[j]))) {
                if k != i && k != j {
                    let mut t = vec![nums[i], nums[j], nums[k]];
                    t.sort_unstable();
                    result.insert(t);
                }
            }
        }

        result.to_owned().into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn case1() {
        assert_eq!(
            Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]), //[-1, 1, -4], [-4, 2, -1], [-1, 0, -1]
            vec![vec![-1, 0, 1], vec![-1, -1, 2]]
        )
    }

    #[test]
    fn case2() {
        assert_eq!(Solution::three_sum(vec![0, 1, 1]), vec![] as Vec<Vec<i32>>)
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
