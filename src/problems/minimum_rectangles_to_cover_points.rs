struct Solution;

// 3111. 覆盖所有点的最少矩形数目
// https://leetcode.cn/problems/minimum-rectangles-to-cover-points

impl Solution {
    pub fn min_rectangles_to_cover_points(points: Vec<Vec<i32>>, w: i32) -> i32 {
        let mut xs: Vec<i32> = points.iter().map(|x| x[0]).collect();
        xs.sort_unstable();
        xs.dedup();

        let mut result = 0;

        let mut left = -1;
        for i in 0..xs.len() {
            let x = xs[i];
            if left == -1 {
                left = x;
                continue;
            }
            let d = x - left;

            if d > w {
                left = x;
                result += 1;
            } else if d == w {
                left = -1;
                result += 1;
            }
        }
        if left != -1 {
            result += 1;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn case_1() {
        let points = [[2, 1], [1, 0], [1, 4], [1, 8], [3, 5], [4, 6]];
        let w = 1;
        let right = 2;
        let result =
            Solution::min_rectangles_to_cover_points(points.map(|x| x.to_vec()).to_vec(), w);
        assert_eq!(result, right);
    }

    #[test]
    fn case_2() {
        let points = [[0, 0], [1, 1], [2, 2], [3, 3], [4, 4], [5, 5], [6, 6]];
        let w = 2;
        let right = 3;
        let result =
            Solution::min_rectangles_to_cover_points(points.map(|x| x.to_vec()).to_vec(), w);
        assert_eq!(result, right);
    }

    #[test]
    fn case_3() {
        let points = [[2, 3], [1, 2]];
        let w = 0;
        let right = 2;
        let result =
            Solution::min_rectangles_to_cover_points(points.map(|x| x.to_vec()).to_vec(), w);
        assert_eq!(result, right);
    }
}
