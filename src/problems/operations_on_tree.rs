// 1993. 树上的操作
// https://leetcode.cn/problems/operations-on-tree

use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
    rc::Rc,
};

#[derive(Debug)]
struct LockingTree {
    table: Vec<State>,
    child_cache: HashMap<i32, Vec<i32>>,
    parent_cache: HashMap<i32, Vec<i32>>,
}

#[derive(Debug, Clone, Copy)]
struct State {
    parent: i32,
    locker: Option<i32>,
}
/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
/**
 * Your LockingTree object will be instantiated and called as such:
 * let obj = LockingTree::new(parent);
 * let ret_1: bool = obj.lock(num, user);
 * let ret_2: bool = obj.unlock(num, user);
 * let ret_3: bool = obj.upgrade(num, user);
 */
impl LockingTree {
    fn new(parent: Vec<i32>) -> Self {
        let len = parent.len();
        Self {
            table: parent
                .into_iter()
                .map(|x| State {
                    parent: x,
                    locker: None,
                })
                .collect(),
            child_cache: HashMap::with_capacity(len),
            parent_cache: HashMap::with_capacity(len),
        }
    }

    fn lock(&mut self, num: i32, user: i32) -> bool {
        let state = &mut self.table[num as usize];
        if state.locker.is_none() {
            state.locker = Some(user);
            true
        } else {
            false
        }
    }

    fn unlock(&mut self, num: i32, user: i32) -> bool {
        let state = &mut self.table[num as usize];

        if state.locker == Some(user) {
            state.locker = None;
            true
        } else {
            false
        }
    }

    fn upgrade(&mut self, num: i32, user: i32) -> bool {
        let state = &mut self.table[num as usize];
        if state.locker.is_none() {
            let chilren = self.get_children(num);
            if chilren.iter().any(|x| x.1.locker.is_some()) {
                let parents = self.get_parents(num);
                if parents.iter().all(|x| x.1.locker.is_none()) {
                    for (i, _) in chilren {
                        self.table[i as usize].locker = None;
                    }
                    self.table[num as usize].locker = Some(user);

                    return true;
                }
            }
        }

        false
    }

    fn get_children(&self, num: i32) -> Vec<(i32, State)> {
        let mut q = VecDeque::with_capacity(self.table.len());
        let mut result = Vec::with_capacity(self.table.len());
        q.push_back(num);

        while let Some(parent) = q.pop_front() {
            let mut children: Vec<(i32, State)> = self
                .table
                .iter()
                .enumerate()
                .filter(|(_, x)| x.parent == parent)
                .map(|(i, &state)| (i as i32, state))
                .collect();
            let mut nums: VecDeque<i32> = children.iter().map(|x| x.0).collect();

            result.append(&mut children);
            q.append(&mut nums);
        }

        result
    }

    fn get_parents(&self, child: i32) -> Vec<(i32, State)> {
        let mut p = child;
        let mut result = Vec::with_capacity(self.table.len());

        while p != -1 {
            let cur = self.table[p as usize];
            result.push((p, cur));
            p = cur.parent;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use std::result;

    use super::LockingTree;

    #[test]
    fn case_1() {
        let opts = ["lock", "unlock", "unlock", "lock", "upgrade", "lock"];
        let init = [-1, 0, 0, 1, 1, 2, 2];
        let inputs = [[2, 2], [2, 3], [2, 2], [4, 5], [0, 1], [0, 1]];
        let right = [true, false, true, true, true, false];

        let mut tree = LockingTree::new(init.into());
        let mut result = Vec::with_capacity(right.len());

        for (opt, input) in opts.into_iter().zip(inputs) {
            let res = match opt {
                "lock" => tree.lock(input[0], input[1]),
                "unlock" => tree.unlock(input[0], input[1]),
                "upgrade" => tree.upgrade(input[0], input[1]),
                _ => panic!(),
            };
            result.push(res);
            println!("opt : {} , input : {:?} , res : {}", opt, &input, res);
            println!("{:?}", &tree.table);
        }
        assert_eq!(result, right);
    }
}
