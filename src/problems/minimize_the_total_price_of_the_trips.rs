struct Solution;

// 2646. 最小化旅行的价格总和
// https://leetcode.cn/problems/minimize-the-total-price-of-the-trips

impl Solution {
    pub fn minimum_total_price(
        n: i32,
        edges: Vec<Vec<i32>>,
        price: Vec<i32>,
        trips: Vec<Vec<i32>>,
    ) -> i32 {
        let n = n as usize;
        let mut g = vec![vec![]; n];
        for e in &edges {
            let x = e[0] as usize;
            let y = e[1] as usize;
            g[x].push(y);
            g[y].push(x);
        }

        fn dfs(x: usize, fa: usize, cnt: &mut Vec<i32>, g: &Vec<Vec<usize>>, end: usize) -> bool {
            if x == end {
                cnt[x] += 1;
                return true; // 找到 end
            }
            for &y in &g[x] {
                if y != fa && dfs(y, x, cnt, g, end) {
                    cnt[x] += 1; // x 是 end 的祖先节点，也就在路径上
                    return true;
                }
            }
            false // 未找到 end
        }
        let mut cnt = vec![0; n];
        for t in &trips {
            dfs(t[0] as usize, n, &mut cnt, &g, t[1] as usize);
        }

        fn dp(
            x: usize,
            fa: usize,
            price: &Vec<i32>,
            cnt: &Vec<i32>,
            g: &Vec<Vec<usize>>,
        ) -> (i32, i32) {
            let mut not_halve = price[x] * cnt[x]; // x 不变
            let mut halve = not_halve / 2; // x 减半
            for &y in &g[x] {
                if y != fa {
                    let (nh, h) = dp(y, x, price, cnt, g); // 计算 y 不变/减半的最小价值总和
                    not_halve += nh.min(h); // x 不变，那么 y 可以不变或者减半，取这两种情况的最小值
                    halve += nh; // x 减半，那么 y 只能不变
                }
            }
            (not_halve, halve)
        }
        let (nh, h) = dp(0, 0, &price, &cnt, &g);
        nh.min(h)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn case_1() {
        let n = 4;
        let edges = [[0, 1], [1, 2], [1, 3]];
        let price = [2, 2, 10, 6];
        let trips = [[0, 3], [2, 1], [2, 3]];

        let right = 23;

        let result = Solution::minimum_total_price(
            n,
            edges.map(|x| x.to_vec()).to_vec(),
            price.to_vec(),
            trips.map(|x| x.to_vec()).to_vec(),
        );

        assert_eq!(result, right)
    }

    #[test]
    fn case_2() {
        let n = 2;
        let edges = [[0, 1]];
        let price = [2, 2];
        let trips = [[0, 0]];

        let right = 1;

        let result = Solution::minimum_total_price(
            n,
            edges.map(|x| x.to_vec()).to_vec(),
            price.to_vec(),
            trips.map(|x| x.to_vec()).to_vec(),
        );

        assert_eq!(result, right)
    }
}
