mod q804 {
    pub fn unique_morse_representations(words: Vec<String>) -> i32 {
        let codes = vec![".-", "-...", "-.-.", "-..", ".", "..-.", "--.", "....", "..", ".---", "-.-", ".-..", "--", "-.", "---", ".--.", "--.-", ".-.", "...", "-", "..-", "...-", ".--", "-..-", "-.--", "--.."];
        let mut set = std::collections::HashSet::new();
        for word in words {
            let mut code = String::new();
            for c in word.chars() {
                code.push_str(codes[c as usize - 97usize]);
            }
            set.insert(code);
        }
        set.len() as i32
    }
}