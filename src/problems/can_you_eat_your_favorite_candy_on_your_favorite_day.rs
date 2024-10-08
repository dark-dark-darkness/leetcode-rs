struct Solution;

//https://leetcode.cn/problems/can-you-eat-your-favorite-candy-on-your-favorite-day/

impl Solution {
    pub fn can_eat(candies_count: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let mut result = vec![false; queries.len()];

        for i in 0..queries.len() {
            println!(
                "================================================================================"
            );

            dbg!(i);
            if let &[favorite_type, favorite_day, daily_cap] = dbg!(queries[i].as_slice()) {
                let count1: i32 = candies_count.iter().take(favorite_type as usize).sum();
                let count2 = count1 + candies_count.get(favorite_type as usize).unwrap_or(&0);
                result[i] = dbg!((favorite_day as u64 + 1) * daily_cap as u64)
                    > dbg!(count1 as u64)
                    && dbg!(favorite_day + 1) < dbg!(count2);
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let candies_count = [7, 4, 5, 3, 8];
        let queries = [[0, 2, 2], [4, 2, 4], [2, 13, 1000000000]];

        let right = [true, false, true];

        let result = Solution::can_eat(
            candies_count.to_vec(),
            queries.into_iter().map(|x| x.to_vec()).collect(),
        );

        assert_eq!(result, right.to_vec());
    }

    #[test]
    fn case_2() {
        let candies_count = [5, 2, 6, 4, 1];
        let queries = [[3, 1, 2], [4, 10, 3], [3, 10, 100], [4, 100, 30], [1, 3, 1]];

        let right = [false, true, true, false, false];

        let result = Solution::can_eat(
            candies_count.to_vec(),
            queries.into_iter().map(|x| x.to_vec()).collect(),
        );

        assert_eq!(result, right.to_vec());
    }

    #[test]
    fn case_3() {
        let candies_count = [
            10, 11, 42, 42, 49, 14, 44, 33, 13, 49, 32, 19, 48, 36, 25, 38, 32, 45, 30, 21, 13, 45,
            39, 12, 12, 25, 26, 18, 35, 28, 1, 31, 14, 16, 38, 49, 26, 33, 39, 39, 7, 31, 20, 8,
            49, 36, 6, 1, 32, 2, 35, 10, 31, 37, 13, 43, 26,
        ];
        let queries = [
            [24, 579, 17],
            [13, 275, 40],
            [38, 432, 75],
            [47, 62, 4],
            [14, 908, 33],
            [19, 1031, 77],
            [18, 316, 71],
            [54, 1558, 48],
            [35, 1403, 19],
            [10, 449, 58],
            [0, 1258, 94],
            [41, 1014, 59],
            [33, 932, 15],
            [18, 1488, 46],
            [51, 630, 89],
            [7, 362, 4],
            [1, 14, 3],
            [0, 1029, 3],
            [2, 1454, 63],
            [52, 19, 44],
            [7, 418, 18],
            [42, 1505, 12],
            [49, 1188, 92],
            [15, 1116, 76],
            [47, 668, 40],
            [50, 468, 7],
            [49, 167, 8],
            [51, 316, 94],
            [27, 1270, 58],
            [1, 158, 66],
            [25, 979, 28],
            [11, 837, 84],
            [27, 1311, 80],
            [16, 1148, 77],
            [51, 1538, 34],
            [19, 120, 70],
            [8, 1508, 7],
            [24, 1464, 93],
            [1, 1448, 44],
            [45, 331, 12],
            [17, 111, 4],
            [6, 332, 19],
            [53, 1368, 98],
            [23, 609, 85],
            [11, 1364, 69],
            [54, 1066, 32],
            [8, 1566, 30],
            [40, 1331, 21],
            [16, 1478, 23],
            [34, 133, 65],
            [17, 1484, 9],
            [37, 1150, 65],
            [13, 885, 69],
            [54, 191, 46],
            [21, 105, 22],
            [1, 37, 75],
            [35, 479, 79],
            [37, 905, 89],
            [49, 551, 74],
            [16, 986, 26],
            [21, 1325, 34],
            [41, 1520, 67],
            [40, 611, 69],
            [7, 997, 22],
            [32, 1108, 39],
            [2, 1549, 59],
            [35, 553, 71],
            [28, 729, 93],
            [15, 357, 11],
            [43, 566, 90],
            [18, 1213, 87],
            [23, 10, 100],
            [8, 423, 18],
            [19, 1270, 59],
            [15, 413, 64],
            [44, 765, 76],
            [5, 17, 97],
            [42, 1228, 10],
            [27, 1236, 44],
            [5, 411, 46],
            [54, 458, 93],
            [27, 1148, 33],
            [20, 429, 85],
            [12, 315, 88],
            [56, 446, 26],
        ];

        let right = [
            true, true, true, false, false, false, true, false, false, false, false, true, true,
            false, true, false, true, false, false, false, false, false, true, false, true, true,
            false, true, false, false, false, false, false, false, false, true, false, false,
            false, true, false, false, true, true, false, true, false, false, false, true, false,
            false, false, true, true, false, true, true, true, false, false, false, true, false,
            false, false, true, true, true, true, false, true, false, false, true, true, true,
            true, false, false, true, false, true, true, true,
        ];

        let result = Solution::can_eat(
            candies_count.to_vec(),
            queries.into_iter().map(|x| x.to_vec()).collect(),
        );

        for i in 0..queries.len() {
            assert!(
                result[i] == right[i],
                "\n{:?}\nqueries {:?}\nleft {:?}\tright {:?}\n",
                i,
                queries[i],
                result[i],
                right[i]
            )
        }
    }

    #[test]
    fn case_4() {
        let candies_count = [
            46, 5, 47, 48, 43, 34, 15, 26, 11, 25, 41, 47, 15, 25, 16, 50, 32, 42, 32, 21, 36, 34,
            50, 45, 46, 15, 46, 38, 50, 12, 3, 26, 26, 16, 23, 1, 4, 48, 47, 32, 47, 16, 33, 23,
            38, 2, 19, 50, 6, 19, 29, 3, 27, 12, 6, 22, 33, 28, 7, 10, 12, 8, 13, 24, 21, 38, 43,
            26, 35, 18, 34, 3, 14, 48, 50, 34, 38, 4, 50, 26, 5, 35, 11, 2, 35, 9, 11, 31, 36, 20,
            21, 37, 18, 34, 34, 10, 21, 8, 5,
        ];
        let queries = [
            [80, 2329, 69],
            [14, 1485, 76],
            [33, 2057, 83],
            [13, 1972, 27],
            [11, 387, 25],
            [24, 1460, 47],
            [22, 1783, 35],
            [1, 513, 33],
            [66, 2124, 85],
            [19, 642, 26],
            [15, 1963, 79],
            [93, 722, 96],
            [15, 376, 88],
            [60, 1864, 89],
            [86, 608, 4],
            [98, 257, 35],
            [35, 651, 47],
            [96, 795, 73],
            [62, 2077, 18],
            [27, 1724, 57],
            [34, 1984, 75],
            [49, 2413, 95],
            [76, 1664, 5],
            [28, 38, 13],
            [85, 54, 42],
            [12, 301, 3],
            [62, 2016, 29],
            [45, 2316, 37],
            [43, 2360, 28],
            [87, 192, 98],
            [27, 2082, 21],
            [74, 762, 37],
            [51, 35, 17],
            [73, 2193, 4],
            [60, 425, 65],
            [11, 1522, 58],
            [21, 1699, 66],
            [42, 1473, 5],
            [30, 2010, 48],
            [91, 796, 74],
            [82, 2162, 31],
            [23, 2569, 65],
            [24, 684, 23],
            [70, 1219, 51],
            [5, 1817, 15],
            [81, 2446, 34],
            [96, 771, 60],
            [49, 1171, 60],
            [41, 567, 67],
            [39, 799, 59],
            [90, 957, 81],
            [84, 2122, 27],
            [82, 1707, 44],
            [11, 1889, 20],
            [80, 1697, 83],
            [24, 1786, 60],
            [90, 1847, 99],
            [51, 114, 21],
            [44, 466, 85],
            [56, 469, 20],
            [44, 350, 96],
            [66, 1946, 10],
            [14, 2470, 12],
            [69, 1175, 18],
            [98, 1804, 25],
            [77, 2187, 40],
            [89, 2265, 45],
            [19, 2246, 45],
            [40, 2373, 79],
            [60, 2222, 17],
            [37, 385, 5],
            [97, 1759, 97],
            [10, 903, 5],
            [87, 842, 45],
            [74, 2398, 66],
            [62, 49, 94],
            [48, 156, 77],
            [76, 2310, 80],
            [64, 2360, 95],
            [70, 1699, 83],
            [39, 1241, 66],
            [92, 2312, 21],
            [63, 2148, 29],
            [95, 594, 74],
            [89, 90, 51],
            [82, 137, 70],
            [54, 301, 97],
            [15, 819, 43],
            [47, 1402, 60],
            [17, 2377, 43],
            [50, 1937, 95],
            [62, 1174, 74],
            [67, 1411, 87],
            [39, 1151, 48],
        ];

        let right = [
            false, false, false, false, true, false, false, false, false, false, false, true, true,
            false, true, true, true, true, false, false, false, false, true, false, true, true,
            false, false, false, true, false, true, false, false, true, false, false, false, false,
            true, true, false, true, true, false, false, true, true, true, true, true, true, true,
            false, true, false, true, true, true, true, true, false, false, true, true, false,
            true, false, false, false, true, true, false, true, false, true, true, false, false,
            true, false, true, false, true, true, true, true, false, true, false, false, true,
            true, true,
        ];

        let result = Solution::can_eat(
            candies_count.to_vec(),
            queries.into_iter().map(|x| x.to_vec()).collect(),
        );

        for i in 0..queries.len() {
            assert!(
                result[i] == right[i],
                "\n{:?}\nqueries {:?}\nleft {:?}\tright {:?}\n",
                i,
                queries[i],
                result[i],
                right[i]
            )
        }
    }
}
