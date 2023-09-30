struct Solution;

// https://leetcode.cn/problems/find-the-losers-of-the-circular-game/

impl Solution {
    pub fn circular_game_losers(n: i32, k: i32) -> Vec<i32> {
        let n = n as usize;
        let step = k as usize;
        let mut k = 0;
        let mut arr = vec![0; n];
        let mut nth = 0;

        loop {
            let i = (nth + k) % n;
            k += step;
            nth = i;
            if arr[i] == 1 {
                break;
            } else {
                arr[i] += 1;
            }
        }

        dbg!(arr)
            .iter()
            .enumerate()
            .filter(|(_, &v)| v == 0)
            .map(|(i, _)| (i + 1) as i32)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let n = 5;
        let k = 2;

        let right = [4, 5];

        let result = Solution::circular_game_losers(n, k);

        assert_eq!(result, right.to_vec());
    }
}
