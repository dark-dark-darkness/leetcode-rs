use std::cell::RefCell;
use std::rc::Rc;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode<T: Clone> {
    pub val: Option<T>,
    pub next: Option<Box<ListNode<T>>>,
}

impl<T: Clone> IntoIterator for ListNode<T> {
    type Item = T;

    type IntoIter = Iter<T>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            current: Some(Box::new(self)),
        }
    }
}

impl<T: Clone> FromIterator<T> for ListNode<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut iter = iter.into_iter();
        if let Some(val) = iter.next() {
            let mut head = Self::from_val(val.clone());

            let mut p = &mut head;

            for v in iter {
                let node = Self::from_val(v.clone());
                p.next = Some(Box::new(node));
                p = p.next.as_mut().unwrap();
            }

            head
        } else {
            Self {
                val: None,
                next: None,
            }
        }
    }
}

impl<T: Clone> ListNode<T> {
    pub fn new() -> Self {
        Self {
            val: None,
            next: None,
        }
    }

    #[inline]
    pub fn from_val(val: T) -> Self {
        Self {
            next: None,
            val: Some(val),
        }
    }

    pub fn from_vec(value: &[T]) -> Self {
        value.iter().cloned().collect()
    }

    pub fn to_vec(&self) -> Vec<T> {
        self.iter().collect()
    }

    fn iter(&self) -> impl Iterator<Item = T> {
        Iter {
            current: Some(Box::new(self.clone())),
        }
    }
}

impl<T: Clone> Default for ListNode<T> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Iter<T: Clone> {
    current: Option<Box<ListNode<T>>>,
}

impl<T: Clone> Iterator for Iter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current.as_ref() {
            None => None,
            Some(current) => {
                let next = current.next.clone();
                let cur_val = current.val.clone();
                self.current = next;
                cur_val
            }
        }
    }
}
