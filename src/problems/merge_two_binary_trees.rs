use std::cell::RefCell;
use std::rc::Rc;

// https://leetcode.cn/problems/merge-two-binary-trees/description/

use crate::struct_def::tree::TreeNode;
struct Solution;

impl Solution {
    pub fn merge_trees(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match (&root1, &root2) {
            (Some(r1), Some(r2)) => {
                let r1_child_left = r1.borrow_mut().left.take();
                let r1_child_right = r1.borrow_mut().right.take();

                let r2_child_left = r2.borrow_mut().left.take();
                let r2_child_right = r2.borrow_mut().right.take();

                Some(Rc::new(RefCell::new(TreeNode {
                    val: r1.borrow().val + r2.borrow().val,
                    left: Self::merge_trees(r1_child_left, r2_child_left),
                    right: Self::merge_trees(r1_child_right, r2_child_right),
                })))
            }
            _ => root1.or(root2),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::struct_def::tree::TreeNode;

    #[test]
    fn case_1() {
        let root1 = [Some(1), Some(3), Some(2), Some(5)];
        let root2 = [Some(2), Some(1), Some(3), None, Some(4), None, Some(7)];

        let right = [Some(3), Some(4), Some(5), Some(5), Some(4), None, Some(7)];

        let result = Solution::merge_trees(TreeNode::from_vec(&root1), TreeNode::from_vec(&root2));

        assert_eq!(TreeNode::to_vec(result), right.to_vec());
    }
}
