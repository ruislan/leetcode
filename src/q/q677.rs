use std::collections::HashMap;

struct Trie {
    children: HashMap<char, Trie>,
    is_word: bool,
    val: i32,
}

struct MapSum {
    trie: Trie,
}


/**
 *  方法1：
 *  字典树存储，
 *  sum的时候，首先按照prefix的值找到根节点，例如“app”那么就是到最后一个'p'的位置，
 *  然后BFS或者DFS都可以，来迭代找到所有的节点，如果是Word的，求和即可
 */
#[allow(unused)]
impl MapSum {

    fn new() -> Self {
        MapSum {
            trie: Trie {
                children: HashMap::new(),
                is_word: false,
                val: 0, 
            },
        }
    }
    
    fn insert(&mut self, key: String, val: i32) {
        let mut cur = &mut self.trie;
        for ch in key.chars() {
            if !cur.children.contains_key(&ch) {
                cur.children.insert(ch, Trie {
                    children: HashMap::new(),
                    is_word: false,
                    val: 0,
                });
            }
            cur = cur.children.get_mut(&ch).unwrap();
        }
        cur.is_word = true;
        cur.val = val;
    }
    
    fn sum(&self, prefix: String) -> i32 {
        let mut cur = &self.trie;
        for ch in prefix.chars() {
            if !cur.children.contains_key(&ch) {
                return 0;
            }
            cur = cur.children.get(&ch).unwrap();
        }
        let mut sum = 0;
        let mut stack = vec![cur];
        while (!stack.is_empty()) {
            let cur = stack.pop().unwrap();
            if cur.is_word {
                sum += cur.val;
            }
            for (_, child) in cur.children.iter() {
                stack.push(child);
            }
        }
        sum
    }
}