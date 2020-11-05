mod q;
mod lcp;
mod offer;
mod interview;
mod sorter;


fn main() {
    println!("{:?} is palindrome", is_ascii_palindrome("abb"));
    println!("{:?} is palindrome", is_ascii_palindrome("abbba"));
    println!("{:?}", slice_to_string_vec(&["hot", "dot", "dog", "lot", "log", "cog"]));
}

pub fn is_ascii_palindrome(s: &str) -> bool {
    let (mut left, mut right) = (0, s.len() - 1);
    while left <= right {
        if s[left..=left] != s[right..=right] { return false; }
        right -= 1;
        left += 1;
    }
    true
}

pub fn slice_to_string_vec(s: &[&str]) -> Vec<String> {
    s.into_iter().map(|x| x.to_string()).collect()
}