use crate::Solution;

// https://leetcode.cn/problems/matrix-diagonal-sum/

impl Solution {
    pub fn diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
        mat.iter()
            .enumerate()
            .flat_map(|(y, r)| {
                r.iter()
                    .enumerate()
                    .filter_map(|(x, &v)| {
                        if x == y || (x + y) == mat.len() - 1 {
                            Some(v)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<i32>>()
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn case_1() {
        let mat = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
        let right = 25;

        let result = Solution::diagonal_sum(mat.map(|x| x.to_vec()).to_vec());

        assert_eq!(result, right);
    }

    #[test]
    fn case_2() {
        let mat = [[1, 1, 1, 1], [1, 1, 1, 1], [1, 1, 1, 1], [1, 1, 1, 1]];
        let right = 8;

        let result = Solution::diagonal_sum(mat.map(|x| x.to_vec()).to_vec());

        assert_eq!(result, right);
    }

    #[test]
    fn case_3() {
        let mat = [[5]];
        let right = 5;

        let result = Solution::diagonal_sum(mat.map(|x| x.to_vec()).to_vec());

        assert_eq!(result, right);
    }
}
