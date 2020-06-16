mod q389 {
    pub fn find_the_difference(s: String, t: String) -> char {
        // 方法1
        // let mut bucket = vec![0i32; 256];
        // for b in s.bytes() {
        //     bucket[b as usize] += 1i32;
        // }
        // for b in t.bytes() {
        //     bucket[b as usize] -= 1i32;
        //     if bucket[b as usize] < 0 {
        //         return b as char;
        //     }
        // }
        // return ' ';

        // 方法2
        // let (mut sum1, mut sum2) = (0i64, 0i64);
        // for b in s.bytes() {
        //     sum1 += b as i64;
        // }
        // for b in t.bytes() {
        //     sum2 += b as i64;
        // }
        // ((sum2 - sum1) as u8) as char

        // 方法3
        let mut res = 0u8;
        for b in s.bytes() {
            res ^= b;
        }
        for b in t.bytes() {
            res ^= b;
        }
        res as char
    }
}