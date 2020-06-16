mod q557 {
    pub fn reverse_words(s: String) -> String {
        let mut s: Vec<&str> = s.split(' ').collect();
        let mut res = String::new();
        for word in s {
            res.push_str(&word.chars().rev().collect::<String>());
            res.push(' ');
        }
        res.pop();
        res
    }
}