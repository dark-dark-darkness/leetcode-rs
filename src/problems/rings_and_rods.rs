struct Solution;

// https://leetcode.cn/problems/rings-and-rods
// 2103. 环和杆

impl Solution {
    pub fn count_points(rings: String) -> i32 {
        let mut table = [[false; 3]; 10];

        let mut iter = rings.bytes();

        while let Some(color) = iter.next() {
            let nth = iter.next().unwrap() - b'0';
            match color {
                b'R' => table[nth as usize][0] = true,
                b'G' => table[nth as usize][1] = true,
                b'B' => table[nth as usize][2] = true,
                _ => panic!(),
            }
        }

        table.iter().filter(|x| x.iter().all(|&x| x)).count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn case_1() {
        let rings = "B0R0G0R9R0B0G0";
        let right = 1;

        let result = Solution::count_points(rings.to_string());

        assert_eq!(result, right)
    }
}
