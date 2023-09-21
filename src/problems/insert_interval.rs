struct Solution;

// https://leetcode.cn/problems/insert-interval/

impl Solution {
    pub fn insert(mut intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        intervals.push(new_interval);
        intervals.sort_unstable_by(|a, b| a[0].cmp(&b[0]));
        let mut result: Vec<Vec<i32>> = Vec::with_capacity(intervals.len());

        for right in intervals {
            match result.last_mut() {
                Some(left) if is_cond((left[0], left[1]), (right[0], right[1])) => {
                    let nums = [left[0], left[1], right[0], right[1]];
                    left[0] = *nums.iter().min().unwrap();
                    left[1] = *nums.iter().max().unwrap();
                }
                _ => result.push(right),
            }
        }

        result
    }
}

fn is_cond((l1, r1): (i32, i32), (l2, r2): (i32, i32)) -> bool {
    (l2 <= r1 && l1 < r2) // l1 .. l2 .. r1 .. r2 
        || (l1 <= r2 && l2 < r1) // l2 .. l1 .. r2 .. r1
        || (l1 <= l2 && r2 <= r1) // l1 .. l2 .. r2 .. r1
        || (l2 <= l1 && r1 <= r2) // l2 .. l1 .. r1 .. r2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let intervals = [[1, 3], [6, 9]];
        let new_interval = [2, 5];
        let right = [[1, 5], [6, 9]];

        let result = Solution::insert(
            intervals.map(|x| x.to_vec()).to_vec(),
            new_interval.to_vec(),
        );

        assert_eq!(result, right.map(|x| x.to_vec()).to_vec())
    }

    #[test]
    fn case_2() {
        let intervals = [[1, 2], [3, 5], [6, 7], [8, 10], [12, 16]];
        let new_interval = [4, 8];
        let right = [[1, 2], [3, 10], [12, 16]];
        
        let result = Solution::insert(
            intervals.map(|x| x.to_vec()).to_vec(),
            new_interval.to_vec(),
        );

        assert_eq!(result, right.map(|x| x.to_vec()).to_vec())
    }

    #[test]
    fn case_3() {
        let intervals: [[i32; 0]; 0] = [];
        let new_interval = [5, 7];
        let right = [[5, 7]];

        let result = Solution::insert(
            intervals.map(|x| x.to_vec()).to_vec(),
            new_interval.to_vec(),
        );

        assert_eq!(result, right.map(|x| x.to_vec()).to_vec())
    }

    #[test]
    fn case_4() {
        let intervals = [[1, 5]];
        let new_interval = [2, 3];
        let right = [[1, 5]];

        let result = Solution::insert(
            intervals.map(|x| x.to_vec()).to_vec(),
            new_interval.to_vec(),
        );

        assert_eq!(result, right.map(|x| x.to_vec()).to_vec())
    }

    #[test]
    fn case_5() {
        let intervals = [[1, 5]];
        let new_interval = [2, 7];
        let right = [[1, 7]];

        let result = Solution::insert(
            intervals.map(|x| x.to_vec()).to_vec(),
            new_interval.to_vec(),
        );

        assert_eq!(result, right.map(|x| x.to_vec()).to_vec())
    }
}
