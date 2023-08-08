use crate::Solution;

// https://leetcode.cn/problems/remove-comments/

impl Solution {
    pub fn remove_comments(source: Vec<String>) -> Vec<String> {
        let mut ans: Vec<String> = Vec::with_capacity(source.len());

        let mut t: Vec<String> = Vec::with_capacity(source.len());

        let mut block_comment = false;

        for s in &source {
            let m = s.len();

            let mut i = 0;

            while i < m {
                if block_comment {
                    if i + 1 < m && &s[i..i + 2] == "*/" {
                        block_comment = false;

                        i += 2;
                    } else {
                        i += 1;
                    }
                } else {
                    if i + 1 < m && &s[i..i + 2] == "/*" {
                        block_comment = true;

                        i += 2;
                    } else if i + 1 < m && &s[i..i + 2] == "//" {
                        break;
                    } else {
                        t.push(s.chars().nth(i).unwrap().to_string());

                        i += 1;
                    }
                }
            }

            if !block_comment && !t.is_empty() {
                ans.push(t.join(""));

                t.clear();
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn case_1() {
        let source = [
            "/*Test program */",
            "int main()",
            "{ ",
            "  // variable declaration ",
            "int a, b, c;",
            "/* This is a test",
            "   multiline  ",
            "   comment for ",
            "   testing */",
            "a = b + c;",
            "}",
        ];
        let right = ["int main()", "{ ", "  ", "int a, b, c;", "a = b + c;", "}"];

        let result = Solution::remove_comments(source.iter().map(|x| x.to_string()).collect());

        assert_eq!(
            result,
            right.iter().map(|x| x.to_string()).collect::<Vec<_>>()
        );
    }

    #[test]
    fn case_2() {
        let source = ["a/*comment", "line", "more_comment*/b"];
        let right = ["ab"];

        let result = Solution::remove_comments(source.iter().map(|x| x.to_string()).collect());

        assert_eq!(
            result,
            right.iter().map(|x| x.to_string()).collect::<Vec<_>>()
        );
    }
}
