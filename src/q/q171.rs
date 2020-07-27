mod q171 {
    pub fn title_to_number(mut s: String) -> i32 {
        let mut res = 0;
        let mut i = 0;
        while let Some(ch) = s.pop() {
            res += 26i32.pow(i) * (ch as u8 - 64) as i32;
            i += 1;
        }
        res as i32
    }
}