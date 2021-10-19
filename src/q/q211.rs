use std::collections::HashMap;

struct WordDictionary {
    trie: Trie,
}

struct Trie {
    children: HashMap<char, Trie>,
    is_word: bool,
}

/**
 * 方法1
 * 字典树的典型应用之一，这里需要注意的就是有'.'这个通配符的情况下，我们就需要考虑到当前所有匹配的字符
 * 这样一来，我们就需要一个stack结构来存储之前的节点和节点对应的word的位置，
 * 这样在处理每一个节点的时候，我们是知道从word的哪个位置开始的
 * AC 24ms 14.1mb 13/13
 **/
#[allow(unused)]
impl WordDictionary {
    fn new() -> Self {
        WordDictionary {
            trie: Trie {
                children: HashMap::new(),
                is_word: false,
            },
        }
    }

    fn add_word(&mut self, word: String) {
        let n = word.len();
        let mut children = &mut self.trie.children;
        for (i, ch) in word.chars().enumerate() {
            let node = children.entry(ch).or_insert(Trie {
                children: HashMap::new(),
                is_word: i == n - 1,
            });
            children = &mut node.children;
        }
    }

    fn search(&self, word: String) -> bool {
        let n = word.len();
        let word = word.chars().collect::<Vec<char>>();
        let mut stack = vec![(&self.trie, 0)];
        while !stack.is_empty() {
            if let Some((trie, i)) = stack.pop() {
                if i == n - 1 {
                    if word[i] == '.' {
                        for (ch, node) in trie.children.iter() {
                            if node.is_word { return true; }
                        }
                    } else {
                        if let Some(node) = trie.children.get(&word[i]) {
                            if node.is_word { return true; }
                        }
                    }
                } else {
                    if word[i] == '.' {
                        for (ch, node) in trie.children.iter() {
                            stack.push((node, i + 1));
                        }
                    } else {
                        if let Some(node) = trie.children.get(&word[i]) {
                            stack.push((node, i + 1));
                        }
                    }
                }
            }
        }
        false
    }
}