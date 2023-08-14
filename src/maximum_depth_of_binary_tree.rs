//Definition for a binary tree node.

// https://leetcode.cn/problems/maximum-depth-of-binary-tree/

use std::{cell::RefCell, rc::Rc};

use crate::{struct_def::tree::TreeNode, Solution};
impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(node) => {
                let left = node.borrow_mut().left.take();
                let right = node.borrow_mut().right.take();
                1 + i32::max(Self::max_depth(left), Self::max_depth(right))
            }
            None => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let vals = [Some(3), Some(9), Some(20), None, None, Some(15), Some(7)];

        let tree = TreeNode::from_vec(&vals);

        let depth = Solution::max_depth(tree);

        assert_eq!(depth, 3)
    }
}
