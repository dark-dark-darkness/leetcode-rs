struct Solution;

// LCP 50. 宝石补给
// https://leetcode.cn/problems/WHnhjV

impl Solution {
    pub fn give_gem(mut gem: Vec<i32>, operations: Vec<Vec<i32>>) -> i32 {
        for opt in operations {
            let (x, y) = (opt[0] as usize, opt[1] as usize);
            let diff = gem[x] / 2;
            gem[x] -= diff;
            gem[y] += diff;
        }

        gem.iter().max().unwrap() - gem.iter().min().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let gem = [3, 1, 2];
        let operations = [[0, 2], [2, 1], [2, 0]];
        let right = 2;

        let result = Solution::give_gem(gem.into(), operations.map(|x| x.into()).into());

        assert_eq!(result, right)
    }

    #[test]
    fn case_2() {
        let gem = [100, 0, 50, 100];
        let operations = [[0, 2], [0, 1], [3, 0], [3, 0]];
        let right = 75;

        let result = Solution::give_gem(gem.into(), operations.map(|x| x.into()).into());

        assert_eq!(result, right)
    }

    #[test]
    fn case_3() {
        let gem = [0, 0, 0, 0];
        let operations = [[1, 2], [3, 1], [1, 2]];
        let right = 0;

        let result = Solution::give_gem(gem.into(), operations.map(|x| x.into()).into());

        assert_eq!(result, right)
    }
}
