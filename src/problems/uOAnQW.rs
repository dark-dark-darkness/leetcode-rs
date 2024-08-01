struct Solution;

// LCP 40. 心算挑战
// https://leetcode.cn/problems/uOAnQW

impl Solution {
    pub fn maxmium_score(cards: Vec<i32>, cnt: i32) -> i32 {
        let mut cards = cards;
        cards.sort_by(|a, b| b.cmp(a));
        let mut ans = 0;
        let mut tmp = 0;
        let mut odd = -1;
        let mut even = -1;
        for i in 0..cnt as usize {
            tmp += cards[i];
            if cards[i] % 2 == 1 {
                odd = cards[i];
            } else {
                even = cards[i];
            }
        }
        if tmp % 2 == 0 {
            return tmp;
        }
        for i in cnt as usize..cards.len() {
            if cards[i] % 2 == 1 {
                if even != -1 {
                    ans = ans.max(tmp - even + cards[i]);
                }
            } else {
                if odd != -1 {
                    ans = ans.max(tmp - odd + cards[i]);
                }
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
        let cards = [1, 2, 8, 9];
        let cnt = 3;
        let right = 18;
        let result = Solution::maxmium_score(cards.to_vec(), cnt);
        assert_eq!(result, right)
    }

    #[test]
    fn case_2() {
        let cards = [3, 3, 1];
        let cnt = 1;
        let right = 0;
        let result = Solution::maxmium_score(cards.to_vec(), cnt);
        assert_eq!(result, right)
    }
}
