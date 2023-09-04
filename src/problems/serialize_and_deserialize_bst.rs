// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//     pub val: i32,
//     pub left: Option<Rc<RefCell<TreeNode>>>,
//     pub right: Option<Rc<RefCell<TreeNode>>>,
// }

// impl TreeNode {
//     #[inline]
//     pub fn new(val: i32) -> Self {
//         TreeNode {
//             val,
//             left: None,
//             right: None,
//         }
//     }
// }
use std::cell::RefCell;
use std::collections::btree_map::Values;
use std::collections::VecDeque;
use std::rc::Rc;

use crate::struct_def::tree::TreeNode;

// https://leetcode.cn/problems/serialize-and-deserialize-bst

struct Codec {}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    fn new() -> Self {
        Self {}
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut nodes: VecDeque<Option<Rc<RefCell<TreeNode>>>> = VecDeque::new();
        let mut result = String::from("[");

        nodes.push_back(root);

        while let Some(node) = nodes.pop_front() {
            if let Some(node) = node {
                let data = node.borrow().val;
                let left = node.borrow_mut().left.take();
                let right = node.borrow_mut().right.take();
                let mut data = data.to_string();
                data.push(',');
                result.push_str(&data);
                nodes.push_back(left);
                nodes.push_back(right);
            } else if nodes.iter().any(|x| x.is_some()) {
                result.push(',');
            }
            println!("{nodes:?}");
            println!("{result:?}");
        }

        if result.len() != 1 {
            result.pop();
        }
        result.push(']');

        result
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        println!("{:?}", data.len());
        if data.len() == 2 {
            return None;
        }

        let nums: Vec<Option<i32>> = data[1..data.len() - 1]
            .split(',')
            .map(|x| match x.parse() {
                Ok(val) => Some(val),
                Err(_) => None,
            })
            .collect();

        let head = Some(Rc::new(RefCell::new(TreeNode::new(nums[0].unwrap()))));
        let mut queue = VecDeque::new();
        queue.push_back(head.as_ref().unwrap().clone());

        for children in nums[1..].chunks(2) {
            let parent = queue.pop_front().unwrap();
            if let [Some(v), ..] = children {
                parent.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(*v))));
                queue.push_back(parent.borrow().left.as_ref().unwrap().clone());
            }
            if let [_, Some(v)] = children {
                parent.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(*v))));
                queue.push_back(parent.borrow().right.as_ref().unwrap().clone());
            }
        }

        println!("{head:?}");
        head
    }
}

// /**
//  * Your Codec object will be instantiated and called as such:
//  * let obj = Codec::new();
//  * let data: String = obj.serialize(strs);
//  * let ans: Option<Rc<RefCell<TreeNode>>> = obj.deserialize(data);
//  */
#[cfg(test)]
mod tests {
    use crate::struct_def::tree::TreeNode;

    use super::Codec;

    #[test]
    fn case_1() {
        let root_vec = vec![Some(2), Some(1), Some(3)];
        let obj = Codec::new();
        let data: String = obj.serialize(TreeNode::from_vec(&root_vec));
        assert_eq!(data, "[2,1,3]");
        let root = obj.deserialize(data);
        assert_eq!(TreeNode::to_vec(root), root_vec);
    }

    #[test]
    fn case_2() {
        let root_vec = vec![];
        let obj = Codec::new();
        let data: String = obj.serialize(TreeNode::from_vec(&root_vec));
        assert_eq!(data, "[]");
        let root = obj.deserialize(data);
        assert_eq!(TreeNode::to_vec(root), root_vec);
    }

    #[test]
    fn case_3() {
        let root_vec = vec![Some(1), None, Some(2)];
        let obj = Codec::new();
        let data: String = obj.serialize(TreeNode::from_vec(&root_vec));
        assert_eq!(data, "[1,,2]");
        let root = obj.deserialize(data);
        assert_eq!(TreeNode::to_vec(root), root_vec);
    }
}
