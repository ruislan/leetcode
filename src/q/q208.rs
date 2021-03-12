// 方法1
// 这个题没啥说的，就是考你教科书的东西了，标准的Tire树实现
// 这里我用了hashmap，当然也可以用数组来解决
// 当然我下面也没有去重构，应该还可以精简代码
// AC 32ms 12.7mb
use std::collections::HashMap;

#[derive(Default)]
struct Trie {
    children: HashMap<char, Trie>,
    is_word: bool,
}

#[allow(unused)]
impl Trie {
    fn new() -> Self {
        Trie::default()
    }

    fn insert(&mut self, word: String) {
        let mut children = &mut self.children;
        for (i, ch) in word.chars().enumerate() {
            let node = children.entry(ch).or_insert(Trie {
                children: HashMap::new(),
                is_word: false,
            });
            if i == word.len() - 1 {
                node.is_word = true;
            }
            children = &mut node.children;
        }
    }

    fn search(&self, word: String) -> bool {
        let mut children = &self.children;
        for (i, ch) in word.chars().enumerate() {
            if let Some(node) = children.get(&ch) {
                if i == word.len() - 1 && !node.is_word {
                    return false;
                }
                children = &node.children;
            } else {
                return false;
            }
        }
        true
    }

    fn starts_with(&self, prefix: String) -> bool {
        let mut children = &self.children;
        for ch in prefix.chars() {
            if let Some(node) = children.get(&ch) {
                children = &node.children;
            } else {
                return false;
            }
        }
        true
    }
}