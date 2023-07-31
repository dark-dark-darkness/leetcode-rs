use std::collections::HashMap;
use std::{error::Error, sync::Mutex};

use once_cell::sync::OnceCell;

use crate::Solution;

static CACHE: OnceCell<Mutex<HashMap<usize, i32>>> = OnceCell::new();

fn get_from_cache<'a>(key: &'a usize) -> Result<Option<i32>, Box<dyn Error>> {
    let mtx = CACHE.get_or_init(|| Mutex::new(HashMap::new())).lock()?;
    let value = mtx.get(key);
    Ok(match value {
        Some(&v) => Some(v),
        None => None,
    })
}

fn set_to_cache(key: &usize, value: &i32) -> Result<(), Box<dyn Error>> {
    let mut mtx = CACHE.get_or_init(|| Mutex::new(HashMap::new())).lock()?;
    mtx.insert(*key, *value);
    Ok(())
}

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        if nums.iter().all(|&n| n <= 0) {
            return *nums.iter().max().unwrap();
        }

        if nums.iter().all(|&n| n > 0) {
            return nums.iter().sum();
        }

        max_sub_array(&nums, nums.len() - 1)
    }
}

fn max_sub_array(nums: &[i32], end_index: usize) -> i32 {
    if end_index == 0 {
        return nums[0];
    }

    if let Some(sum) = get_from_cache(&end_index).unwrap() {
        return sum;
    }

    let sum = max_sub_array(nums, end_index - 1);
    let sum = if nums[end_index] > 0 {
        sum + nums[end_index]
    } else {
        sum
    };

    set_to_cache(&end_index, &sum).ok();

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]),
            6
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::max_sub_array(vec![1]), 1);
    }

    #[test]
    fn case_3() {
        assert_eq!(Solution::max_sub_array(vec![5, 4, -1, 7, 8]), 23);
    }

    #[test]
    fn case_4() {
        assert_eq!(Solution::max_sub_array(vec![-2, 1]), 1);
    }

    #[test]
    fn case_5() {
        assert_eq!(
            Solution::max_sub_array(vec![
                -9, 9, 7, -8, -3, 2, -9, 1, 7, -2, -9, 7, -9, 5, 5, -3, 9, 0
            ]),
            16
        );
    }

    #[test]
    fn case_6() {
        assert_eq!(
            Solution::max_sub_array(vec![
                -84, -87, -78, -16, -94, -36, -87, -93, -50, -22, -63, -28, -91, -60, -64, -27,
                -41, -27, -73, -37, -12, -69, -68, -30, -83, -31, -63, -24, -68, -36, -30, -3, -23,
                -59, -70, -68, -94, -57, -12, -43, -30, -74, -22, -20, -85, -38, -99, -25, -16,
                -71, -14, -27, -92, -81, -57, -74, -63, -71, -97, -82, -6, -26, -85, -28, -37, -6,
                -47, -30, -14, -58, -25, -96, -83, -46, -15, -68, -35, -65, -44, -51, -88, -9, -77,
                -79, -89, -85, -4, -52, -55, -100, -33, -61, -77, -69, -40, -13, -27, -87, -95,
                -40
            ]),
            -3
        );
    }

    #[test]
    fn case_7() {
        assert_eq!(Solution::max_sub_array(vec![-1]), -1);
    }
}
