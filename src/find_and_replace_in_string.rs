use crate::Solution;

// https://leetcode.cn/problems/find-and-replace-in-string/

impl Solution {
    pub fn find_replace_string(
        mut s: String,
        indices: Vec<i32>,
        sources: Vec<String>,
        targets: Vec<String>,
    ) -> String {
        let n = indices.len();
        let mut arr = vec![];

        for i in 0..n {
            arr.push((indices[i] as usize, i));
        }

        arr.sort_unstable();

        for (i, j) in arr.into_iter().rev() {
            let source = &sources[j];
            let m = source.len();
            let target = &targets[j];

            if i + m <= s.len() && &s[i..i + m] == source {
                s.replace_range(i..i + m, target);
            }
        }

        s
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn case_1() {
        let s = "abcd";
        let indices = [0, 2];
        let sources = ["a", "cd"];
        let targets = ["eee", "ffff"];

        let right = "eeebffff";

        let result: String = Solution::find_replace_string(
            s.to_string(),
            indices.to_vec(),
            sources.map(|x| x.to_string()).to_vec(),
            targets.map(|x| x.to_string()).to_vec(),
        );

        assert_eq!(result, right.to_string());
    }

    #[test]
    fn case_2() {
        let s = "abcd";
        let indices = [0, 2];
        let sources = ["ab", "ec"];
        let targets = ["eee", "ffff"];

        let right = "eeebffff";

        let result: String = Solution::find_replace_string(
            s.to_string(),
            indices.to_vec(),
            sources.map(|x| x.to_string()).to_vec(),
            targets.map(|x| x.to_string()).to_vec(),
        );

        assert_eq!(result, right.to_string());
    }
}
