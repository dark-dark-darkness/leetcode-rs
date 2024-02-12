struct Solution;
// 2706. 购买两块巧克力
// https://leetcode.cn/problems/buy-two-chocolates

impl Solution {
    pub fn buy_choco(prices: Vec<i32>, money: i32) -> i32 {
        let mut mins = [i32::MAX - 1, i32::MAX];

        for price in prices {
            if price < mins[0] {
                mins[1] = mins[0];
                mins[0] = price;
            } else if price == mins[0] {
                mins[1] = price
            } else if price < mins[1] {
                mins[1] = price;
            }
        }

        let sum = mins[0] + mins[1];

        if sum > money {
            money
        } else {
            money - sum
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn case_1() {
        let prices = [1, 2, 2];
        let money = 3;

        let right = 0;

        let result = Solution::buy_choco(prices.to_vec(), money);

        assert_eq!(result, right)
    }
}
