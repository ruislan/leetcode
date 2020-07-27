mod q451 {
    pub fn frequency_sort(s: String) -> String {
        let mut bucket = vec![(0, 0); 256];
        for b in s.bytes() {
            bucket[b as usize].0 += 1;
            bucket[b as usize].1 = b;
        }
        bucket.sort_by(|a, b| b.0.cmp(&a.0));
        let mut res = String::new();
        for (times, ch) in bucket {
            for _ in 0..times {
                res.push(ch as char);
            }
        }
        res
    }
}