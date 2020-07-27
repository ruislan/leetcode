mod q762 {
    pub fn count_prime_set_bits(l: i32, r: i32) -> i32 {
        let l = l as usize;
        let r = r as usize;
        let mut sum = 0;
        for i in l..=r {
            let num = i.count_ones();
            match num {
                2 | 3 | 5 | 7 | 11 | 13 | 17 | 19 | 23 | 29 | 31 => sum += 1,
                _ => continue,
            }
        }
        sum
    }
}