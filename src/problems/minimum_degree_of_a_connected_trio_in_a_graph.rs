use crate::Solution;

// https://leetcode.cn/problems/minimum-degree-of-a-connected-trio-in-a-graph/

impl Solution {
    pub fn min_trio_degree(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        use std::collections::HashSet;
        let n = n as usize;
        let mut graph: Vec<HashSet<usize>> = vec![HashSet::with_capacity(n + 1); n + 1];

        for edge in edges {
            let a = edge[0] as usize;
            let b = edge[1] as usize;
            graph[a].insert(b);
            graph[b].insert(a);
        }

        println!("{graph:?}");

        let mut result = usize::MAX;

        for (a, nodes) in graph.iter().enumerate().skip(1) {
            for &b in nodes {
                for &c in nodes {
                    if b != c && graph[b].contains(&c) && graph[c].contains(&b) {
                        result = graph[a]
                            .iter()
                            .chain(graph[b].iter())
                            .chain(graph[c].iter())
                            .filter(|&&x| x != b && x != c && x != a)
                            .count()
                            .min(result);

                        if result == 0 {
                            return 0;
                        }
                    }
                }
            }
        }

        if result == usize::MAX {
            -1
        } else {
            result as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn case1() {
        let n = 6;
        let edges = [[1, 2], [1, 3], [3, 2], [4, 1], [5, 2], [3, 6]];
        let right = 3;

        let result = Solution::min_trio_degree(n, edges.map(|x| x.to_vec()).to_vec());

        assert_eq!(result, right)
    }

    #[test]
    fn case_2() {
        let n = 7;
        let edges = [
            [1, 3],
            [4, 1],
            [4, 3],
            [2, 5],
            [5, 6],
            [6, 7],
            [7, 5],
            [2, 6],
        ];
        let right = 0;

        let result = Solution::min_trio_degree(n, edges.map(|x| x.to_vec()).to_vec());

        assert_eq!(result, right)
    }

    #[test]
    fn case_3() {
        let n = 6;
        let edges = [[1, 2], [1, 3], [3, 2], [4, 1], [5, 2], [3, 6]];
        let right = 3;

        let result = Solution::min_trio_degree(n, edges.map(|x| x.to_vec()).to_vec());

        assert_eq!(result, right)
    }

    #[test]
    fn case_4() {
        let n = 400;
        let edges: Vec<[i32; 2]> =
            include!("minimum_degree_of_a_connected_trio_in_a_graph.case_4.txt");
        let right = 3;

        let result = Solution::min_trio_degree(n, edges.into_iter().map(|x| x.to_vec()).collect());

        assert_eq!(result, right)
    }
}
