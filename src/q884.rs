mod q884 {
    pub fn uncommon_from_sentences(a: String, b: String) -> Vec<String> {
        let mut map = std::collections::HashMap::new();
        for word in a.split_whitespace() {
            let mut count = map.entry(word).or_insert(0);
            *count += 1;
        }
        for word in b.split_whitespace() {
            let mut count = map.entry(word).or_insert(0);
            *count += 1;
        }

        let mut res = Vec::new();
        for (k, v) in map.iter() {
            if *v == 1 { res.push(String::from(*k)); }
        }
        res
    }
}