struct Solution;
// 2332. 坐上公交的最晚时间
// https://leetcode.cn/problems/the-latest-time-to-catch-a-bus

impl Solution {
    pub fn latest_time_catch_the_bus(
        mut buses: Vec<i32>,
        mut passengers: Vec<i32>,
        capacity: i32,
    ) -> i32 {
        buses.sort_unstable();
        passengers.sort_unstable();

        // 模拟乘客上车
        let mut j = 0;
        let mut c = 0;
        for &t in &buses {
            c = capacity;
            while c > 0 && j < passengers.len() && passengers[j] <= t {
                j += 1;
                c -= 1;
            }
        }

        // 寻找插队时机
        j = j.saturating_sub(1);
        let mut ans = if c > 0 {
            *buses.last().unwrap()
        } else {
            passengers[j]
        };
        while j < passengers.len() && ans == passengers[j] {
            ans -= 1; // 往前找没人到达的时刻
            j = j.saturating_sub(1);
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let buses = [10, 20];
        let passengers = [2, 17, 18, 19];
        let capacity = 2;
        let right = 16;
        let result =
            Solution::latest_time_catch_the_bus(buses.to_vec(), passengers.to_vec(), capacity);
        assert_eq!(result, right);
    }

    #[test]
    fn case_2() {
        let buses = [20, 30, 10];
        let passengers = [19, 13, 26, 4, 25, 11, 21];
        let capacity = 2;
        let right = 20;
        let result =
            Solution::latest_time_catch_the_bus(buses.to_vec(), passengers.to_vec(), capacity);
        assert_eq!(result, right);
    }

    #[test]
    fn case_3() {
        let buses = [3];
        let passengers = [2, 3];
        let capacity = 2;
        let right = 1;
        let result =
            Solution::latest_time_catch_the_bus(buses.to_vec(), passengers.to_vec(), capacity);
        assert_eq!(result, right);
    }

    #[test]
    fn case_4() {
        let buses = [3];
        let passengers = [2, 4];
        let capacity = 2;
        let right = 3;
        let result =
            Solution::latest_time_catch_the_bus(buses.to_vec(), passengers.to_vec(), capacity);
        assert_eq!(result, right);
    }

    #[test]
    fn case_5() {
        let buses = [2, 3];
        let passengers = [3, 2];
        let capacity = 2;
        let right = 1;
        let result =
            Solution::latest_time_catch_the_bus(buses.to_vec(), passengers.to_vec(), capacity);
        assert_eq!(result, right);
    }
}
