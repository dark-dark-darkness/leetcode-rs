use std::collections::HashMap;

struct Solution;

// 746. 使用最小花费爬楼梯
// https://leetcode.cn/problems/min-cost-climbing-stairs

impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        min_cost_climbing_stairs(&cost, cost.len(), &mut HashMap::new())
    }
}

fn min_cost_climbing_stairs(cost: &Vec<i32>, n: usize, cache: &mut HashMap<usize, i32>) -> i32 {
    if let Some(res) = cache.get(&n) {
        res.clone()
    } else {
        let result = match n {
            1 => cost[0],
            2 => cost[1].min(cost[0]),
            3 => i32::min(cost[0] + cost[2], cost[1]),
            n => i32::min(
                cost[n - 1] + min_cost_climbing_stairs(cost, n - 1, cache),
                cost[n - 2] + min_cost_climbing_stairs(cost, n - 2, cache),
            ),
        };

        cache.insert(n, result);

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn case_1() {
        let cost = [1, 100, 1, 1, 1, 100, 1, 1, 100, 1];
        let right = 6;

        let result = Solution::min_cost_climbing_stairs(cost.to_vec());

        assert_eq!(result, right);
    }

    #[test]
    fn case_2() {
        let cost = [10, 15, 20];
        let right = 15;

        let result = Solution::min_cost_climbing_stairs(cost.to_vec());

        assert_eq!(result, right);
    }

    #[test]
    fn case_3() {
        let cost = [0, 1, 1, 1];
        let right = 1;

        let result = Solution::min_cost_climbing_stairs(cost.to_vec());

        assert_eq!(result, right);
    }
}
