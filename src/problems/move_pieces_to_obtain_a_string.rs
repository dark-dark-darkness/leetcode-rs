struct Solution;

// https://leetcode.cn/problems/move-pieces-to-obtain-a-string

impl Solution {
    pub fn can_change(start: String, target: String) -> bool {
        let (start, target) = (start.as_bytes(), target.as_bytes());
        let len = if start.len() == target.len() {
            start.len()
        } else {
            return false;
        };

        let (mut i, mut j) = (0, 0);

        while i < len && j < len {
            match (start[i], target[j]) {
                (b'L', b'L') if i >= j => {
                    i += 1;
                    j += 1;
                }
                (b'R', b'R') if i <= j => {
                    i += 1;
                    j += 1;
                }
                (b'_', _) => {
                    i += 1;
                }
                (_, b'_') => {
                    j += 1;
                }
                _ => {
                    return false;
                }
            }
        }

        while i < len && start[i] == b'_' {
            i += 1;
        }
        while j < len && target[j] == b'_' {
            j += 1;
        }

        (i == len) && (i == j)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let start = "_L__R__R_";
        let target = "L______RR";

        let right = true;

        let result = Solution::can_change(start.to_string(), target.to_string());

        assert_eq!(result, right);
    }

    #[test]
    fn case_2() {
        let start = "R_L_";
        let target = "__LR";

        let right = false;

        let result = Solution::can_change(start.to_string(), target.to_string());

        assert_eq!(result, right);
    }

    #[test]
    fn case_3() {
        let start = "_R";
        let target = "R_";

        let right = false;

        let result = Solution::can_change(start.to_string(), target.to_string());

        assert_eq!(result, right);
    }
}
