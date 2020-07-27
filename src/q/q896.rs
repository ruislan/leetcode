mod q896 {
    pub fn is_monotonic(a: Vec<i32>) -> bool {
        let mut up = None;
        for i in 1..a.len() {
            if a[i - 1] == a[i] { continue; }
            if a[i - 1] > a[i] {
                if up == None { up = Some(false); }
                if up != Some(false) { return false; }
            } else {
                if up == None { up = Some(true); }
                if up != Some(true) { return false; }
            }
        }
        true
    }
}