mod q633 {
    pub fn judge_square_sum(c: i32) -> bool {
        let mut a = 0i64;
        let mut b = (c as f64).sqrt().ceil() as i64;
        while a <= b {
            let mut sum = a * a + b * b;
            if sum < c as i64 {
                a += 1;
            } else if sum > c as i64 {
                b -= 1;
            } else {
                return true;
            }
        }
        false
    }
}