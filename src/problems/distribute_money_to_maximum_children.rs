struct Solution;

// 2591. 将钱分给最多的儿童
// https://leetcode.cn/problems/distribute-money-to-maximum-children/

impl Solution {
    pub fn dist_money(mut money: i32, mut children: i32) -> i32 {
        if money == children * 8 {
            return children;
        }

        if money < children {
            return -1;
        }

        if money <= children + 6 {
            return 0;
        }

        let mut result = 0;

        loop {
            match children {
                0 => break,
                1 => match money {
                    4 => {
                        result -= 1;
                        break;
                    }
                    8 => {
                        result += 1;
                        break;
                    }
                    _ => {
                        break;
                    }
                },
                _ => {
                    if money >= 8 {
                        result += 1;
                        children -= 1;
                        money -= 8
                    } else if money == children * 4 {
                        result -= 1;
                        break;
                    } else if money < children {
                        result -= (children - money) / 7;
                        break;
                    } else if money < children + 7 {
                        break;
                    }
                }
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
        let money = 20;
        let children = 3;
        let right = 1;

        let result = Solution::dist_money(money, children);

        assert_eq!(result, right)
    }

    #[test]
    fn case_2() {
        let money = 16;
        let children = 2;
        let right = 2;

        let result = Solution::dist_money(money, children);

        assert_eq!(result, right)
    }

    #[test]
    fn case_3() {
        let money = 10;
        let children = 3;
        let right = 1;

        let result = Solution::dist_money(money, children);

        assert_eq!(result, right)
    }

    #[test]
    fn case_4() {
        let money = 11;
        let children = 3;
        let right = 2;

        let result = Solution::dist_money(money, children);

        assert_eq!(result, right)
    }
}
