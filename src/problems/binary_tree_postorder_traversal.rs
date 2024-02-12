use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use crate::struct_def::tree::TreeNode;

struct Solution;

// 145. 二叉树的后序遍历
// https://leetcode.cn/problems/binary-tree-postorder-traversal/

impl Solution {
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        if let Some(root) = root {
            let mut s = VecDeque::new();

            let mut result = Vec::with_capacity(100);

            s.push_back(Some(root.clone()));

            while let Some(node) = s.pop_back() {
                if let Some(node) = node {
                    s.push_back(node.borrow().left.clone());
                    s.push_back(node.borrow().right.clone());
                    result.push(node.borrow().val);
                }
            }
            result.reverse();
            result
        } else {
            vec![]
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::struct_def::tree::TreeNode;

    use super::Solution;

    #[test]
    fn case_1() {
        let root = TreeNode::from_vec(&[Some(1), None, Some(2), Some(3)]);
        let result = Solution::postorder_traversal(root);
        let right = vec![3, 2, 1];
        assert_eq!(result, right);
    }

    #[test]
    fn case_2() {
        let root = TreeNode::from_vec(&[]);
        let result = Solution::postorder_traversal(root);
        let right: Vec<i32> = vec![];
        assert_eq!(result, right);
    }

    #[test]
    fn case_3() {
        let root = TreeNode::from_vec(&[Some(1)]);
        let result = Solution::postorder_traversal(root);
        let right: Vec<i32> = vec![1];
        assert_eq!(result, right);
    }
}
