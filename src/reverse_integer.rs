use crate::Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let xs = x.to_string();

        let mut flag = 1;

        let x: Vec<u8> = if x < 0 {
            flag = -1;
            xs.bytes().skip(1)
        } else {
            xs.bytes().skip(0)
        }
        .rev()
        .skip_while(|&b| b == b'0')
        .collect();

        let x = &String::from_utf8_lossy(&x);

        println!("{x}");

        i32::from_str_radix(x, 10).unwrap_or(0) * flag
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let res = Solution::reverse(123);
        assert_eq!(res, 321);
    }

    #[test]
    fn case_2() {
        let res = Solution::reverse(1230);
        assert_eq!(res, 321);
    }

    #[test]
    fn case_3() {
        let res = Solution::reverse(-1230);
        assert_eq!(res, -321);
    }
}
