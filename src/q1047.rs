mod q1047 {
    pub fn remove_duplicates(s: String) -> String {
        let mut stack = Vec::new();
        for ch in s.chars() {
            if stack.is_empty() {
                stack.push(ch);
            } else {
                if let Some(&x) = stack.last() {
                    if x == ch { stack.pop(); } else { stack.push(ch); }
                }
            }
        }
        stack.iter().collect()
    }
}