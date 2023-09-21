struct Solution;

// 2603. 收集树中金币
// https://leetcode.cn/problems/collect-coins-in-a-tree/

impl Solution {
    pub fn collect_the_coins(coins: Vec<i32>, edges: Vec<Vec<i32>>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn case_1() {
        let coins = [1, 0, 0, 0, 0, 1];
        let edges = [[0, 1], [1, 2], [2, 3], [3, 4], [4, 5]];
        let right = 2;
        let result = Solution::collect_the_coins(coins.into(), edges.map(|x| x.into()).into());
        assert_eq!(result, right);
    }
}
