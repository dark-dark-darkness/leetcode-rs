use crate::Solution;

// https://leetcode.cn/problems/number-of-ways-of-cutting-a-pizza/

impl Solution {
    pub fn ways(pizza: Vec<String>, k: i32) -> i32 {
        let m = pizza.len();
        let n = pizza[0].len();
        let k = k as usize;
        const MOD: i32 = 1e9 as i32 + 7;
        let mut apples = vec![vec![0; n + 1]; m + 1];
        let mut db = vec![vec![vec![0; n + 1]; m + 1]; k + 1];

        for i in (0..m).rev() {
            for j in (0..n).rev() {
                apples[i][j] = apples[i + 1][j] + apples[i][j + 1] - apples[i + 1][j + 1]
                    + (if pizza[i].as_bytes()[j] == b'A' { 1 } else { 0 });
                db[1][i][j] = if apples[i][j] > 0 { 1 } else { 0 };
            }
        }

        for k in 2..=k {
            for i in 0..m {
                for j in 0..n {
                    for x in (i + 1)..m {
                        if apples[i][j] > apples[x][j] {
                            db[k][i][j] = (db[k][i][j] + db[k - 1][x][j]) % MOD;
                        }
                    }
                    for y in (j + 1)..n {
                        if apples[i][j] > apples[i][y] {
                            db[k][i][j] = (db[k][i][j] + db[k - 1][i][y]) % MOD;
                        }
                    }
                }
            }
        }

        db[k][0][0]
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn case_1() {
        let pizza = ["A..", "AAA", "..."];
        let k = 3;

        let right = 3;

        let result = Solution::ways(pizza.map(|x| x.to_string()).to_vec(), k);

        assert_eq!(result, right)
    }

    #[test]
    fn case_2() {
        let pizza = ["A..", "AA.", "..."];
        let k = 3;

        let right = 1;

        let result = Solution::ways(pizza.map(|x| x.to_string()).to_vec(), k);

        assert_eq!(result, right)
    }

    #[test]
    fn case_3() {
        let pizza = ["A..", "A..", "..."];
        let k = 1;

        let right = 1;

        let result = Solution::ways(pizza.map(|x| x.to_string()).to_vec(), k);

        assert_eq!(result, right)
    }
}
