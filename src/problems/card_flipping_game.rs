use crate::Solution;
use std::collections::HashSet;

// https://leetcode.cn/problems/card-flipping-game

impl Solution {
    pub fn flipgame(fronts: Vec<i32>, backs: Vec<i32>) -> i32 {
        let forb: HashSet<i32> = fronts
            .iter()
            .zip(backs.iter())
            .filter_map(|(&a, &b)| if a == b { Some(a) } else { None })
            .collect();

        fronts
            .into_iter()
            .chain(backs.into_iter())
            .filter(|x| !forb.contains(x))
            .min()
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn case_1() {
        let fronts = [1, 2, 4, 4, 7];
        let backs = [1, 3, 4, 1, 3];
        let res = Solution::flipgame(fronts.into(), backs.into());
        assert_eq!(res, 2);
    }

    #[test]
    fn case_2() {
        let fronts = [1];
        let backs = [1];
        let res = Solution::flipgame(fronts.into(), backs.into());
        assert_eq!(res, 0);
    }
}
