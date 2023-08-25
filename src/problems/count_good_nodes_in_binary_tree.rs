use std::cell::RefCell;
use std::rc::Rc;

// https://leetcode.cn/problems/count-good-nodes-in-binary-tree/

use crate::struct_def::tree::TreeNode;
use crate::Solution;

impl Solution {
    pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        good_nodes_rec(&root, i32::MIN)
    }
}

fn good_nodes_rec(root: &Option<Rc<RefCell<TreeNode>>>, mut high: i32) -> i32 {
    if let Some(n) = root {
        let v = n.borrow().val;
        let c = if v >= high {
            high = v;
            1
        } else {
            0
        };
        c + good_nodes_rec(&n.borrow().left, high) + good_nodes_rec(&n.borrow().right, high)
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use std::{
        cell::{Ref, RefCell},
        rc::Rc,
    };

    use crate::{struct_def::tree::TreeNode, Solution};

    #[test]
    fn case_1() {
        let root = [Some(3), Some(1), Some(4), Some(3), None, Some(1), Some(5)];
        let right = 4;

        let result = Solution::good_nodes(TreeNode::from_vec(&root));

        assert_eq!(result, right);
    }

    #[test]
    fn case_2() {
        let root = [Some(3), Some(3), None, Some(4), Some(2)];
        let right = 3;

        let result = Solution::good_nodes(TreeNode::from_vec(&root));

        assert_eq!(result, right);
    }
}
