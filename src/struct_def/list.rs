#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode<T: Clone> {
    pub val: Option<T>,
    pub next: Option<Box<ListNode<T>>>,
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

    pub fn from_vec(value: &Vec<T>) -> Self {
        value.to_owned().into()
    }

    pub fn into_vec(&self) -> Vec<T> {
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
