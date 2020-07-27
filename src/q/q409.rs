mod q409 {
    pub fn longest_palindrome(s: String) -> i32 {
        let mut bucket = vec![0i32; 256];
        for b in s.bytes() {
            bucket[b as usize] += 1;
        }
        let mut max_len = 0i32;
        for v in bucket {
            max_len += (v / 2i32) * 2i32;
        }
        if max_len != s.len() as i32 { max_len + 1 } else { max_len }
    }
}