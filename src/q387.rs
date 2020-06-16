mod q387 {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut bucket = vec![0; 256];
        for ch in s.bytes() {
            bucket[ch as usize] += 1;
        }
        for (i, ch) in s.bytes().enumerate() {
            if bucket[ch as usize] == 1 {
                return i as i32;
            }
        }
        -1i32
    }
}