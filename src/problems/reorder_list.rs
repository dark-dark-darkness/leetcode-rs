use crate::struct_def::list::ListNode;

struct Solution;

// https://leetcode.cn/problems/reorder-list/

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
    use super::*;
    use crate::struct_def::list::ListNode;

    #[test]
    fn case_1() {
        let raw = vec![1, 2, 3, 4, 5];
        let right = vec![1, 5, 2, 4, 3];

        let head = &mut Some(Box::new(ListNode::from_vec(&raw)));
        Solution::reorder_list(head);
        assert_eq!(head.as_ref().unwrap().to_vec(), right);
    }

    #[test]
    fn case_2() {
        let raw = vec![1, 2, 3, 4];
        let right = vec![1, 4, 2, 3];

        let head = &mut Some(Box::new(ListNode::from_vec(&raw)));
        Solution::reorder_list(head);
        assert_eq!(head.as_ref().unwrap().to_vec(), right);
    }
}
