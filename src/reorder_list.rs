use crate::Solution;

// https://leetcode.cn/problems/reorder-list/

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode<T: Clone> {
    pub val: Option<T>,
    pub next: Option<Box<ListNode<T>>>,
}

type LinkedList<T> = ListNode<T>;

impl<T: Clone> ListNode<T> {
    fn new() -> Self {
        Self {
            val: None,
            next: None,
        }
    }

    #[inline]
    fn from_val(val: T) -> Self {
        Self {
            next: None,
            val: Some(val),
        }
    }

    fn from_vec(value: &Vec<T>) -> Self {
        value.to_owned().into()
    }

    fn into_vec(&self) -> Vec<T> {
        self.to_owned().into()
    }
}

impl<T: Clone> From<Vec<T>> for ListNode<T> {
    fn from(value: Vec<T>) -> Self {
        if value.len() == 0 {
            return Self {
                val: None,
                next: None,
            };
        }

        let mut head = Self::from_val(value[0].clone());

        let mut p = &mut head;

        for v in value[1..].iter() {
            let node = Self::from_val(v.clone());
            p.next = Some(Box::new(node));
            p = p.next.as_mut().unwrap();
        }

        head
    }
}

impl<T: Clone> Into<Vec<T>> for ListNode<T> {
    fn into(self) -> Vec<T> {
        let mut result = vec![];

        let mut p = &Some(Box::new(self.clone()));

        loop {
            if let Some(v) = p {
                result.push(v.val.as_ref().unwrap().clone());
                p = &v.next;
            } else {
                break;
            }
        }
        result
    }
}

impl Solution {
    pub fn reorder_list(head: &mut Option<Box<ListNode<i32>>>) {
        let mut new_head = Box::new(ListNode::new());

        let values = {
            let mut result = vec![];

            let mut p = &head.clone();

            while let Some(v) = p {
                result.push(v.val);
                p = &v.as_ref().next;
            }

            result
        };

        let mut lp = 0;
        let mut rp = values.len() - 1;
        let mut p = &mut new_head;

        loop {
            let node = ListNode::from_val(values[lp].unwrap_or(0));
            lp += 1;
            p.next = Some(Box::new(node));
            p = p.next.as_mut().unwrap();

            if lp > rp {
                break;
            }

            let node = ListNode::from_val(values[rp].unwrap_or(0));
            rp -= 1;
            p.next = Some(Box::new(node));
            p = p.next.as_mut().unwrap();

            if lp > rp {
                break;
            }
        }

        *head = new_head.next;
    }
}

#[cfg(test)]
mod tests {
    use crate::{reorder_list::ListNode, Solution};

    #[test]
    fn case_1() {
        let raw = vec![1, 2, 3, 4, 5];
        let right = vec![1, 5, 2, 4, 3];

        let head = &mut Some(Box::new(ListNode::from_vec(&raw)));
        Solution::reorder_list(head);
        assert_eq!(head.as_ref().unwrap().into_vec(), right);
    }

    #[test]
    fn case_2() {
        let raw = vec![1, 2, 3, 4];
        let right = vec![1, 4, 2, 3];

        let head = &mut Some(Box::new(ListNode::from_vec(&raw)));
        Solution::reorder_list(head);
        assert_eq!(head.as_ref().unwrap().into_vec(), right);
    }
}
