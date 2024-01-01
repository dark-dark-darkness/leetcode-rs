struct Solution;

impl Solution {
    pub fn make_smallest_palindrome(s: String) -> String {
        let mut chars: Vec<u8> = s.bytes().collect();
        let len = chars.len();
        for i in 0..len / 2 {
            let l = i;
            let r = len - i - 1;
            match u8::cmp(&chars[l], &chars[r]) {
                std::cmp::Ordering::Less => chars[r] = chars[l],
                std::cmp::Ordering::Equal => {}
                std::cmp::Ordering::Greater => chars[l] = chars[r],
            }
        }

        String::from_utf8_lossy(&chars).to_string()
    }
}
