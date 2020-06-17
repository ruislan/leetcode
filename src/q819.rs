mod q819 {
    pub fn most_common_word(paragraph: String, banned: Vec<String>) -> String {
        let mut set = std::collections::HashSet::new();
        for ban in banned {
            set.insert(ban);
        }
        let mut dic = std::collections::HashMap::new();
        let mut word = String::new();
        for ch in paragraph.chars() {
            if ch.is_ascii_alphabetic() {
                word.push(ch.to_ascii_lowercase());
            } else {
                if !word.is_empty() && !set.contains(&word) {
                    let h = dic.entry(word.clone()).or_insert(0);
                    *h += 1;
                }
                word = String::new();
            }
        }
        if !word.is_empty() && !set.contains(&word) {
            let h = dic.entry(word).or_insert(0);
            *h += 1;
        }
        let (mut w, mut n) = (String::new(), 0);
        for (k, v) in dic {
            if v > n {
                w = k.clone();
                n = v;
            }
        }
        w
    }
}