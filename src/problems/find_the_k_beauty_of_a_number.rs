struct Solution;
// 2269. 找到一个数字的 K 美丽值
// https://leetcode.cn/problems/find-the-k-beauty-of-a-number
impl Solution {
    pub fn divisor_substrings(num: i32, k: i32) -> i32 {
        let num_str = num.to_string();
        let num_chars: Vec<u8> = num_str.bytes().collect();
        let len = num_str.len();
        let k = k as usize;
        let p: i32 = 10_i32.pow(k as u32 - 1);
        let mut n = str::parse::<i32>(&num_str[0..k]).unwrap();
        let mut ans = if num % n == 0 { 1 } else { 0 };

        for i in 1..(len - k + 1) {
            let o = (num_chars[i + k - 1] - b'0') as i32;
            let s = (n % p) * 10;
            n = s + o;
            if n != 0 {
                ans += if num % n == 0 { 1 } else { 0 };
            }
        }

        ans
    }
}
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn case_1() {
        let (num, k) = (240, 2);
        let right = 2;
        let result = Solution::divisor_substrings(num, k);
        assert_eq!(result, right)
    }

    #[test]
    fn case_2() {
        let (num, k) = (430043, 2);
        let right = 2;
        let result = Solution::divisor_substrings(num, k);
        assert_eq!(result, right)
    }
}
