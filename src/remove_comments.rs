use crate::Solution;

impl Solution {
    pub fn remove_comments(source: Vec<String>) -> Vec<String> {
        let mut ans: Vec<String> = Vec::new();

        let mut t: Vec<String> = Vec::new();

        let mut blockComment = false;

        for s in &source {
            let m = s.len();

            let mut i = 0;

            while i < m {
                if blockComment {
                    if i + 1 < m && &s[i..i + 2] == "*/" {
                        blockComment = false;

                        i += 2;
                    } else {
                        i += 1;
                    }
                } else {
                    if i + 1 < m && &s[i..i + 2] == "/*" {
                        blockComment = true;

                        i += 2;
                    } else if i + 1 < m && &s[i..i + 2] == "//" {
                        break;
                    } else {
                        t.push(s.chars().nth(i).unwrap().to_string());

                        i += 1;
                    }
                }
            }

            if !blockComment && !t.is_empty() {
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
