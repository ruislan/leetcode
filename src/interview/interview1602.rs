use std::collections::HashMap;

struct WordsFrequency {
    freq: HashMap<String, i32>,
}

// 方法1
// 这个就是用hashmap统计一次即可
// 128ms 29.8mb
#[allow(unused)]
impl WordsFrequency {
    fn new(book: Vec<String>) -> Self {
        let mut freq = HashMap::new();
        for b in book {
            *freq.entry(b).or_insert(0) += 1;
        }
        WordsFrequency { freq }
    }
    fn get(&self, word: String) -> i32 {
        *self.freq.get(&word).or(Some(&0)).unwrap()
    }
}
