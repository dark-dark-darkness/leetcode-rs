use std::cell::RefCell;
use std::rc::Rc;

// https://leetcode.cn/problems/lowest-common-ancestor-of-deepest-leaves/

use crate::struct_def::tree::TreeNode;
struct Solution;

impl Solution {
    pub fn lca_deepest_leaves(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>) -> (Option<Rc<RefCell<TreeNode>>>, i32) {
            if let Some(rr) = root {
                let tem = rr.borrow();
                let (l, ld) = dfs(tem.left.clone());
                let (r, rd) = dfs(tem.right.clone());
                if ld > rd {
                    return (l, ld + 1);
                }
                if ld < rd {
                    return (r, rd + 1);
                }
                (Some(rr.clone()), ld + 1)
            } else {
                (root, 0)
            }
        }
        dfs(root).0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::struct_def::tree::TreeNode;

    #[test]
    fn case_1() {
        let root = [
            Some(3),
            Some(5),
            Some(1),
            Some(6),
            Some(2),
            Some(0),
            Some(8),
            None,
            None,
            Some(7),
            Some(4),
        ];
        let right = [2, 7, 4];

        let result = Solution::lca_deepest_leaves(TreeNode::from_vec(&root));

        assert_eq!(TreeNode::to_vec(result), right.map(Some).to_vec());
    }

    #[test]
    fn case_2() {
        let root = [Some(1)];
        let right = [1];

        let result = Solution::lca_deepest_leaves(TreeNode::from_vec(&root));

        assert_eq!(TreeNode::to_vec(result), right.map(Some).to_vec());
    }

    #[test]
    fn case_3() {
        let root = [Some(0), Some(1), Some(3), None, Some(2)];
        let right = [2];

        let result = Solution::lca_deepest_leaves(TreeNode::from_vec(&root));

        assert_eq!(TreeNode::to_vec(result), right.map(Some).to_vec());
    }

    #[test]
    fn case_4() {
        let root = [
            Some(1),
            Some(2),
            None,
            Some(3),
            Some(4),
            None,
            Some(6),
            None,
            Some(5),
        ];
        let right = [Some(2), Some(3), Some(4), None, Some(6), None, Some(5)];

        let result = Solution::lca_deepest_leaves(TreeNode::from_vec(&root));

        assert_eq!(TreeNode::to_vec(result), right.to_vec());
    }
}
