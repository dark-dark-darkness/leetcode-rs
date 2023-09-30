struct Solution;

// 2603. 收集树中金币
// https://leetcode.cn/problems/collect-coins-in-a-tree/

impl Solution {
    pub fn collect_the_coins(coins: Vec<i32>, edges: Vec<Vec<i32>>) -> i32 {
        let n = coins.len();
        let mut g = vec![Vec::with_capacity(n); n];
        let mut deg = vec![0; n];
        for e in &edges {
            let x = e[0] as usize;
            let y = e[1] as usize;
            g[x].push(y);
            g[y].push(x); // 建图
            deg[x] += 1;
            deg[y] += 1; // 统计每个节点的度数（邻居个数）
        }

        let mut left_edges = n as i32 - 1; // 剩余边数

        // 拓扑排序，去掉没有金币的子树
        let mut q = Vec::with_capacity(n);
        for i in 0..n {
            if deg[i] == 1 && coins[i] == 0 {
                // 没有金币的叶子
                q.push(i);
            }
        }
        while !q.is_empty() {
            left_edges -= 1; // 删除节点到其父节点的边
            for &y in &g[q.pop().unwrap()] {
                deg[y] -= 1;
                if deg[y] == 1 && coins[y] == 0 {
                    // 没有金币的叶子
                    q.push(y);
                }
            }
        }

        // 再次拓扑排序
        for i in 0..n {
            if deg[i] == 1 && coins[i] == 1 {
                // 有金币的叶子（判断 coins[i] 是避免把没有金币的叶子也算进来）
                q.push(i);
            }
        }
        left_edges -= q.len() as i32; // 删除所有叶子（到其父节点的边）
        for &x in &q {
            // 遍历所有叶子
            for &y in &g[x] {
                deg[y] -= 1;
                if deg[y] == 1 {
                    // y 现在是叶子了
                    left_edges -= 1; // 删除 y（到其父节点的边）
                }
            }
        }
        0.max(left_edges * 2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let coins = [1, 0, 0, 0, 0, 1];
        let edges = [[0, 1], [1, 2], [2, 3], [3, 4], [4, 5]];
        let right = 2;
        let result = Solution::collect_the_coins(coins.into(), edges.map(|x| x.into()).into());
        assert_eq!(result, right);
    }

    #[test]
    fn case_2() {
        let coins = [0, 0, 0, 1, 1, 0, 0, 1];
        let edges = [[0, 1], [0, 2], [1, 3], [1, 4], [2, 5], [5, 6], [5, 7]];
        let right = 2;
        let result = Solution::collect_the_coins(coins.into(), edges.map(|x| x.into()).into());
        assert_eq!(result, right);
    }
}
