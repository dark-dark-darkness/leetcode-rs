use std::{cell::RefCell, collections::VecDeque, rc::Rc};

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

    pub fn to_vec(tree: Option<Rc<RefCell<TreeNode<T>>>>) -> Vec<Option<T>> {
        let mut result = vec![];
        let mut q = VecDeque::new();
        q.push_back(tree);
        while let Some(n) = q.pop_front() {
            if let Some(n) = n {
                let n = n.borrow();
                result.push(Some(n.val.clone()));
                match (&n.left, &n.right) {
                    (Some(left), Some(right)) => {
                        q.push_back(Some(Rc::clone(left)));
                        q.push_back(Some(Rc::clone(right)));
                    }
                    (Some(left), None) => {
                        q.push_back(Some(Rc::clone(left)));
                        q.push_back(None);
                    }
                    (None, Some(right)) => {
                        q.push_back(None);
                        q.push_back(Some(Rc::clone(right)));
                    }
                    (None, None) => {}
                }
            } else {
                result.push(None)
            }
        }
        result
    }
}
