use std::collections::VecDeque;

struct LRUCache {
    list: VecDeque<(i32, i32)>,
    capacity: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */

/**
 * Your LRUCache object will be instantiated and called as such:
 * let obj = LRUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */
impl LRUCache {
    fn new(capacity: i32) -> Self {
        Self {
            list: VecDeque::with_capacity(capacity as usize),
            capacity: capacity as usize,
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        match self.list.iter().enumerate().find(|(_, (k, _))| *k == key) {
            Some((i, &(key, val))) => {
                self.list.remove(i);
                self.list.push_back((key, val));

                val
            }
            None => -1,
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        match self.list.iter().enumerate().find(|(_, (k, _))| *k == key) {
            Some((i, _)) => {
                self.list.remove(i);
                self.list.push_back((key, value));
            }
            None => {
                if self.list.len() == self.capacity {
                    self.list.pop_front();
                }
                self.list.push_back((key, value));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LRUCache;
    enum Opt {
        Get(i32),
        Put(i32, i32),
    }

    #[test]
    fn case_1() {
        let capacity = 2;
        let mut cache = LRUCache::new(capacity);
        let opts = [
            Opt::Put(1, 1),
            Opt::Put(2, 2),
            Opt::Get(1),
            Opt::Put(3, 3),
            Opt::Get(2),
            Opt::Put(4, 4),
            Opt::Get(1),
            Opt::Get(3),
            Opt::Get(4),
        ];
        let right = [1, -1, -1, 3, 4];

        let mut result = Vec::with_capacity(capacity as usize);

        for opt in opts {
            match opt {
                Opt::Get(k) => result.push(cache.get(k)),
                Opt::Put(k, v) => cache.put(k, v),
            }
        }

        assert_eq!(result, right);
    }
}
