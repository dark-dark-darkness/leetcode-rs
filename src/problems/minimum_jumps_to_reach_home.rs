use crate::Solution;
// https://leetcode.cn/problems/minimum-jumps-to-reach-home/

impl Solution {
    pub fn minimum_jumps(forbidden: Vec<i32>, a: i32, b: i32, x: i32) -> i32 {
        use std::collections::HashSet;
        use std::collections::VecDeque;
        let mut que: VecDeque<(i32, i32, i32)> = [(0, 1, 0)].into();
        let mut visited: HashSet<i32> = [0].into();
        let max = i32::max(*forbidden.iter().max().unwrap() + a, x) + b;
        let min = 0;
        let forbidden: HashSet<i32> = forbidden.into_iter().collect();

        let can_move_fn =
            |pos: i32, dire: i32, forbidden: &HashSet<i32>, visited: &HashSet<i32>| {
                min <= pos
                    && pos <= max
                    && !visited.contains(&(pos * dire))
                    && !forbidden.contains(&pos)
            };

        while let Some((position, direction, step)) = que.pop_front() {
            if position == x {
                return step;
            }

            let next = position + a;
            let next_dire = 1;

            if can_move_fn(next, next_dire, &forbidden, &visited) {
                visited.insert(next * next_dire);
                que.push_back((next, next_dire, step + 1))
            }

            if direction == 1 {
                let next = position - b;
                let next_dire = -1;
                if can_move_fn(next, next_dire, &forbidden, &visited) {
                    visited.insert(next * next_dire);
                    que.push_back((next, next_dire, step + 1))
                }
            }
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn case_1() {
        let forbidden = [14, 4, 18, 1, 15];
        let a = 3;
        let b = 15;
        let x = 9;

        let right = 3;

        let result = Solution::minimum_jumps(forbidden.to_vec(), a, b, x);

        assert_eq!(result, right);
    }

    #[test]
    fn case_2() {
        let forbidden = [8, 3, 16, 6, 12, 20];
        let a = 15;
        let b = 13;
        let x = 11;

        let right = -1;

        let result = Solution::minimum_jumps(forbidden.to_vec(), a, b, x);

        assert_eq!(result, right);
    }

    #[test]
    fn case_3() {
        let forbidden = [1, 6, 2, 14, 5, 17, 4];
        let a = 16;
        let b = 9;
        let x = 7;

        let right = 2;

        let result = Solution::minimum_jumps(forbidden.to_vec(), a, b, x);

        assert_eq!(result, right);
    }

    #[test]
    fn case_4() {
        let forbidden = [128, 178, 147, 165, 63, 11, 150, 20, 158, 144, 136];
        let a = 61;
        let b = 170;
        let x = 135;

        let right = 6;

        let result = Solution::minimum_jumps(forbidden.to_vec(), a, b, x);

        assert_eq!(result, right);
    }
}
