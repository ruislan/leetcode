mod q686 {
    pub fn repeated_string_match(a: String, b: String) -> i32 {
        // 方法1
        // let mut s = a.clone();
        // let mut count = 1;
        // while !s.contains(&b) {
        //     if s.len() > b.len() * 2 && count > 2 { return -1; }
        //     count += 1;
        //     s.push_str(&a);
        // }
        // count

        // 方法2
        // let mut s = a.clone();
        // let mut sum = 1;
        // while s.len() < b.len() {
        //     s.push_str(&a);
        //     sum += 1;
        // }
        // let mut count = 0;
        // while !s.contains(&b) {
        //     if count > 2 { return -1; }
        //     s.push_str(&a);
        //     count += 1;
        // }
        // sum + count

        // 方法3
        let mut s = a.clone();
        let mut sum = 1;
        while s.len() < b.len() {
            s.push_str(&a);
            sum += 1;
        }

        if s.contains(&b) {
            sum
        } else {
            s.push_str(&a);
            if s.contains(&b) {
                sum + 1
            } else {
                -1
            }
        }
    }
}