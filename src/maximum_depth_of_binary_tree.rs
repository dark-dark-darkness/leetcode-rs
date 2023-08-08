//Definition for a binary tree node.

// https://leetcode.cn/problems/maximum-depth-of-binary-tree/

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode<T = i32> {
    pub val: T,
    pub left: Option<Rc<RefCell<TreeNode<T>>>>,
    pub right: Option<Rc<RefCell<TreeNode<T>>>>,
}

impl<T: Clone> TreeNode<T> {
    #[inline]
    pub fn new(val: T) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    pub fn from_vec(vec: &[Option<T>]) -> Option<Rc<RefCell<TreeNode<T>>>> {
        use std::collections::VecDeque;
        let head = Some(Rc::new(RefCell::new(TreeNode::new(
            vec[0].clone().unwrap(),
        ))));
        let mut queue = VecDeque::new();
        queue.push_back(head.as_ref().unwrap().clone());

        for children in vec[1..].chunks(2) {
            let parent = queue.pop_front().unwrap();
            if let [Some(v), ..] = children {
                parent.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(v.clone()))));
                queue.push_back(parent.borrow().left.as_ref().unwrap().clone());
            }
            if let [_, Some(v)] = children {
                parent.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(v.clone()))));
                queue.push_back(parent.borrow().right.as_ref().unwrap().clone());
            }
        }
        head
    }
}

use std::{cell::RefCell, rc::Rc};

use crate::Solution;
impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Solution::max_depth_rec(&root)
    }

    pub fn max_depth_rec(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root {
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
        let vals = [Some(3), Some(9), Some(20), None, None, Some(15), Some(7)];

        let tree = TreeNode::from_vec(&vals);

        let depth = Solution::max_depth(tree);

        assert_eq!(depth, 3)
    }
}
