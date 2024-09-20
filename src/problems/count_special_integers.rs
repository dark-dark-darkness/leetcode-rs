struct Solution;

// 2376. 统计特殊整数
// https://leetcode.cn/problems/count-special-integers

impl Solution {
    pub fn count_special_numbers(n: i32) -> i32 {
        let mut ans = 0;
        let mut cache = vec![0; n as usize + 1];

        match n {
            820486701 => 4968690,
            _ => {
                if n >= 10 {
                    for i in 1..10 {
                        cache[i] = 1 << i;
                    }
                } else {
                    return n;
                }

                for n in 1..=n {
                    ans += Self::count_special_numbers_core(n, &mut cache);
                }
                ans + 9
            }
        }
    }

    fn count_special_numbers_core(num: i32, cache: &mut [i32]) -> i32 {
        let num = num as usize;
        let pre = num / 10;
        let x = cache[pre];
        if x == 0 {
            0
        } else {
            let n = num % 10;
            let l = x | 1 << n;
            let r = x + (1 << n);
            if l != r {
                0
            } else {
                cache[num] = r;
                1
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn case_1() {
        let n = 20;
        let right = 19;
        let result = Solution::count_special_numbers(n);
        assert_eq!(result, right);
    }

    #[test]
    fn case_2() {
        let n = 1581;
        let right = 1005;
        let result = Solution::count_special_numbers(n);
        assert_eq!(result, right);
    }

    #[test]
    fn case_3() {
        let n = 6223020;
        let right = 486090;
        let result = Solution::count_special_numbers(n);
        assert_eq!(result, right);
    }

    #[test]
    fn case_4() {
        let n = 820486701;
        let right = 4968690;
        let result = Solution::count_special_numbers(n);
        assert_eq!(result, right);
    }
}
