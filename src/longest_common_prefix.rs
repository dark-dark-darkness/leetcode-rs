use crate::Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        strs.iter()
            .max()
            .unwrap()
            .chars()
            .zip(strs.iter().min().unwrap().chars())
            .take_while(|x| x.0 == x.1)
            .map(|x| x.0)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn case1() {
        assert_eq!(
            Solution::longest_common_prefix(
                ["flower", "flow", "flight"]
                    .iter()
                    .map(|x| x.to_string())
                    .collect(),
            ),
            "fl".to_string()
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            Solution::longest_common_prefix(
                ["dog", "racecar", "car"]
                    .iter()
                    .map(|x| x.to_string())
                    .collect(),
            ),
            "".to_string()
        );
    }
}
