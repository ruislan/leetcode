mod q482 {
    pub fn license_key_formatting(s: String, k: i32) -> String {
        let k = k as usize;
        let mut s = s;
        let mut i = 0;
        let mut res = String::new();
        while let Some(ch) = s.pop() {
            if ch != '-' {
                res.push(ch.to_ascii_uppercase());
                i += 1;
                if i % k == 0 {
                    res.push('-');
                }
            }
        }
        if i > 0 && i % k == 0 {
            res.pop();
        }
        res.chars().rev().collect::<String>()
    }
}