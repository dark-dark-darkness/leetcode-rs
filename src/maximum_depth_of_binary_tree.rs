//Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
use std::cell::RefCell;
use std::rc::Rc;

use crate::Solution;
impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Solution::max_depth_rec(&root)
    }

    pub fn max_depth_rec(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root {
            let node = Rc::clone(&node);
            let node = node.borrow();
            1 + i32::max(
                Solution::max_depth_rec(&node.left),
                Solution::max_depth_rec(&node.right),
            )
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        //[3,9,20,null,null,15,7]
        let tree = TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 20,
                left: Some(Rc::new(RefCell::new(TreeNode::new(15)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
            }))),
        };

        let depth = Solution::max_depth(Some(Rc::new(RefCell::new(tree))));

        assert_eq!(depth, 3)
    }
}
