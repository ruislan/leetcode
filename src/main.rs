fn main() {
    let s = "abbba";
    let m = s.len() / 2;
    println!("{:?}", &s[..=m]);
    println!("{:?}", &s[m..]);
    println!("{:?}", &s.chars().rev().collect::<String>()[..=m]);
    println!("{:?} is palindrome", is_palindrome("abb"));
}

fn is_palindrome(s: &str) -> bool {
    let (mut left, mut right) = (0, s.len() - 1);
    while left <= right {
        if s[left..=left] != s[right..=right] { return false; }
        right -= 1;
        left += 1;
    }
    true
}
