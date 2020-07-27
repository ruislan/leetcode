mod q1078 {
    pub fn find_ocurrences(text: String, first: String, second: String) -> Vec<String> {
        let vec_text = text.split_ascii_whitespace().collect::<Vec<&str>>();
        let mut res = Vec::new();
        for i in 2..vec_text.len() {
            if &first == vec_text[i - 2] && &second == vec_text[i - 1] {
                res.push(vec_text[i].to_string());
            }
        }
        res
    }
}