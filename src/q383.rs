mod q383 {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut c = vec![0; 256];
        for b in magazine.bytes() {
            c[b as usize] += 1;
        }
        for b in ransom_note.bytes() {
            c[b as usize] -= 1;
            if c[b as usize] < 0 { return false; }
        }
        true
    }
}