use crate::Solution;

//Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        Self { next: None, val }
    }

    fn from_vec(data: &Vec<i32>) -> Self {
        if data.len() == 0 {
            return Self { val: 0, next: None };
        }

        let mut head = Self {
            val: data[0],
            next: None,
        };

        let mut p = &mut head;

        for &v in data[1..].into_iter() {
            let node = Self::new(v);
            p.next = Some(Box::new(node));
            p = p.next.as_mut().unwrap();
        }

        head
    }

    fn into_vec(&self) -> Vec<i32> {
        let mut result = vec![];

        let mut p = &Some(Box::new(self.clone()));

        loop {
            if let Some(v) = p {
                result.push(v.val);
                p = &v.next;
            } else {
                break;
            }
        }
        result
    }
}

impl Solution {
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        let mut new_head = Box::new(ListNode { val: 0, next: None });

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
            let node = ListNode::new(values[lp]);
            lp += 1;
            p.next = Some(Box::new(node));
            p = p.next.as_mut().unwrap();

            if lp > rp {
                break;
            }

            let node = ListNode::new(values[rp]);
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
