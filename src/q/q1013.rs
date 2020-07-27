mod q1013 {
    pub fn can_three_parts_equal_sum(a: Vec<i32>) -> bool {
        let sum: i32 = a.iter().sum();
        if sum % 3 != 0 { return false; }
        let exp = sum / 3;
        let mut m = 0;
        let mut parts = 0;
        for i in 0..a.len() - 1 {
            m += a[i];
            if m == exp {
                m = 0;
                parts += 1;
                if parts == 2 { return true; }
            }
        }
        false
    }
}