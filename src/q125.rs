mod q125 {
    pub fn is_palindrome(s: String) -> bool {
        let mut stack: Vec<char> = Vec::new();
        for mut ch in s.chars() {
            if ch.is_ascii_digit() || ch.is_ascii_alphabetic() {
                ch.make_ascii_lowercase();
                stack.push(ch);
            }
        }
        let len = stack.len();
        for i in 0..len / 2 {
            if stack[i] != stack[len - 1 - i] { return false; }
        }
        true
    }
}