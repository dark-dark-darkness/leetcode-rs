struct Solution;

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

// 337. 打家劫舍 III
// https://leetcode.cn/problems/house-robber-iii

use crate::struct_def::tree::TreeNode;
impl Solution {
    pub fn roc_rec(node: Option<Rc<RefCell<TreeNode>>>) -> [i32; 2] {
        if let Some(n) = node {
            let dp_l = Self::roc_rec(n.borrow().left.clone());
            let dp_r = Self::roc_rec(n.borrow().right.clone());
            [
                dp_l[1] + dp_r[1],
                (dp_l[0] + dp_r[0] + n.borrow().val).max(dp_l[1] + dp_r[1]),
            ]
        } else {
            [0; 2]
        }
    }

    pub fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::roc_rec(root)[1]
    }
}
