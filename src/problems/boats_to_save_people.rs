struct Solution;

impl Solution {
    pub fn num_rescue_boats(mut people: Vec<i32>, limit: i32) -> i32 {
        people.sort_unstable();
        let mut ans = 0;
        let (mut i, mut j) = (0, people.len() - 1);
        while i < j {
            if people[i] + people[j] <= limit {
                i += 1;
            }
            j -= 1;
            ans += 1;
        }
        ans + if i == j { 1 } else { 0 }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn case_1() {
        let people = [1, 2];
        let limit = 3;
        let result = Solution::num_rescue_boats(people.to_vec(), limit);
        let right = 1;
        assert_eq!(result, right)
    }

    #[test]
    fn case_2() {
        let people = [3, 2, 2, 1];
        let limit = 3;
        let right = 3;
        let result = Solution::num_rescue_boats(people.to_vec(), limit);
        assert_eq!(result, right)
    }

    #[test]
    fn case_3() {
        let people = [3, 5, 3, 4];
        let limit = 5;
        let right = 4;
        let result = Solution::num_rescue_boats(people.to_vec(), limit);
        assert_eq!(result, right)
    }
}
