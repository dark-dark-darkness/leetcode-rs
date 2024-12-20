use std::collections::HashMap;

struct Solution;

// 3138. 同位字符串连接的最小长度
// https://leetcode.cn/problems/minimum-length-of-anagram-concatenation
impl Solution {
    pub fn min_anagram_length(s: String) -> i32 {
        let bs: Vec<u8> = s.bytes().collect();
        let len = bs.len();
        let half = len / 2;
        let mut cnt: HashMap<u8, i32> = (b'a'..=b'z').map(|x| (x, 0)).collect();

        for i in 1..=half {
            let c = bs[i - 1];
            *cnt.get_mut(&c).unwrap() += 1;
            if len % i != 0 {
                continue;
            }
            let all = bs[i..].chunks(i).all(|x| {
                x.iter().fold(
                    (b'a'..=b'z').map(|x| (x, 0)).collect::<HashMap<u8, i32>>(),
                    |mut acc, c| {
                        *acc.get_mut(c).unwrap() += 1;
                        acc
                    },
                ) == cnt
            });

            if all {
                return i as i32;
            }
        }

        len as i32
    }
}
