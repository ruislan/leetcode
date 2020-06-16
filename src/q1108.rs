mod q1108 {
    pub fn defang_i_paddr(address: String) -> String {
        let mut res = String::new();
        for ch in address.chars() {
            if ch == '.' { res.push_str("[.]"); } else { res.push(ch); }
        }
        res
    }
}