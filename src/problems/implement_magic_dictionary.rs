// 676. 实现一个魔法字典
// https://leetcode.cn/problems/implement-magic-dictionary

struct MagicDictionary {
    set: Vec<String>,
}

/**
 * Your MagicDictionary object will be instantiated and called as such:
 * let obj = MagicDictionary::new();
 * obj.build_dict(dictionary);
 * let ret_2: bool = obj.search(searchWord);
 */

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MagicDictionary {
    fn new() -> Self {
        MagicDictionary { set: Vec::new() }
    }

    fn build_dict(&mut self, dictionary: Vec<String>) {
        self.set = dictionary;
    }

    fn search(&self, search_word: String) -> bool {
        self.set
            .iter()
            .filter(|x| x.len() == search_word.len())
            .any(|x| {
                x.chars()
                    .zip(search_word.chars())
                    .filter(|(a, b)| a != b)
                    .count()
                    == 1
            })
    }
}
