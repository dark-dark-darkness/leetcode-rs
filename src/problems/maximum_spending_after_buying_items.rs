struct Solution;

// 2931. 购买物品的最大开销
// https://leetcode.cn/problems/maximum-spending-after-buying-items/
impl Solution {
    pub fn max_spending(values: Vec<Vec<i32>>) -> i64 {
        let m = values.len();
        let n = values[0].len();
        let mut lasts = vec![Some(n - 1); m];
        let mut result = 0;

        for d in 1..=m * n {
            let (s, i) = lasts
                .iter()
                .enumerate()
                .filter_map(|(s, i)| i.map(|i| (s, i)))
                .min_by_key(|(s, i)| values[*s][*i])
                .unwrap();
            result += d as i64 * values[s][i] as i64;

            match lasts[s] {
                Some(0) => lasts[s] = None,
                Some(v) => lasts[s] = Some(v - 1),
                None => {}
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn case_1() {
        let values = [[8, 5, 2], [6, 4, 1], [9, 7, 3]];
        let right = 285;
        let result = Solution::max_spending(values.map(|x| x.to_vec()).to_vec());
        assert_eq!(result, right);
    }

    #[test]
    fn case_2() {
        let values = [[10, 8, 6, 4, 2], [9, 7, 5, 3, 2]];
        let right = 386;
        let result = Solution::max_spending(values.map(|x| x.to_vec()).to_vec());
        assert_eq!(result, right);
    }
}
