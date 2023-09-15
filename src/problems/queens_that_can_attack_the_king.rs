use crate::Solution;

// 1222. 可以攻击国王的皇后
// https://leetcode.cn/problems/queens-that-can-attack-the-king/

impl Solution {
    pub fn queens_attackthe_king(mut queens: Vec<Vec<i32>>, king: Vec<i32>) -> Vec<Vec<i32>> {
        queens.sort_unstable_by(|a, b| {
            get_dest((king[0], king[1]), (a[0], a[1]))
                .cmp(&get_dest((king[0], king[1]), (b[0], b[1])))
        });

        [
            queens.iter().find(|x| x[0] == king[0] && x[1] > king[1]),
            queens.iter().find(|x| x[0] == king[0] && x[1] < king[1]),
            queens.iter().find(|x| x[1] == king[1] && x[0] > king[0]),
            queens.iter().find(|x| x[1] == king[1] && x[0] < king[0]),
            queens.iter().find(|x| {
                (x[0] - king[0]).abs() == (x[1] - king[1]).abs() && x[0] > king[0] && x[1] > king[1]
            }),
            queens.iter().find(|x| {
                (x[0] - king[0]).abs() == (x[1] - king[1]).abs() && x[0] < king[0] && x[1] > king[1]
            }),
            queens.iter().find(|x| {
                (x[0] - king[0]).abs() == (x[1] - king[1]).abs() && x[0] > king[0] && x[1] < king[1]
            }),
            queens.iter().find(|x| {
                (x[0] - king[0]).abs() == (x[1] - king[1]).abs() && x[0] < king[0] && x[1] < king[1]
            }),
        ]
        .into_iter()
        .flatten()
        .map(|x| vec![x[0], x[1]])
        .collect()
    }
}

fn get_dest(a: (i32, i32), b: (i32, i32)) -> i32 {
    (a.0 - b.0).pow(2) + (a.1 - b.1).pow(2)
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn case_1() {
        let queens = [[0, 1], [1, 0], [4, 0], [0, 4], [3, 3], [2, 4]];
        let king = [0, 0];
        let right = [[0, 1], [1, 0], [3, 3]];

        let result = Solution::queens_attackthe_king(queens.map(|x| x.into()).into(), king.into());

        assert_eq!(result, right)
    }
}
