mod q1137 {
    pub fn tribonacci(n: i32) -> i32 {
        let (mut t0, mut t1, mut t2) = (0, 1, 1);
        return match n {
            0 => t0,
            1 => t1,
            2 => t2,
            _ => {
                let mut sum = 0;
                for i in 3..=n {
                    sum = t0 + t1 + t2;
                    t0 = t1;
                    t1 = t2;
                    t2 = sum;
                }
                sum
            }
        };
    }
}