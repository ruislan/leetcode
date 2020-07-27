mod q717 {
    pub fn is_one_bit_character(bits: Vec<i32>) -> bool {
        let mut i = 0;
        while i < bits.len() {
            let bit = bits[i];
            if i == bits.len() - 1 && bit == 0 { return true; }
            if bit == 0 { i += 1; } else { i += 2; }
        }
        false
    }
}