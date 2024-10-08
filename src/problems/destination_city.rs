struct Solution;

// 1436. 旅行终点站
// https://leetcode.cn/problems/destination-city/
impl Solution {
    pub fn dest_city(paths: Vec<Vec<String>>) -> String {
        use std::collections::HashMap;

        let mut dic: HashMap<String, i32> = HashMap::with_capacity(paths.len() / 2 + 1);

        for p in paths {
            let from = p[0].clone();
            let to = p[1].clone();

            if let Some(v) = dic.get_mut(&from) {
                *v += 10;
            } else {
                dic.insert(from, 10);
            }

            if let Some(v) = dic.get_mut(&to) {
                *v += 1;
            } else {
                dic.insert(to, 1);
            }
        }

        for (key, value) in dic {
            if value == 1 {
                return key;
            }
        }

        "".to_string()
    }
}
