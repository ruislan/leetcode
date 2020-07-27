mod q202 {
    pub fn is_happy(n: i32) -> bool {
        let mut n = n;
        let mut result = 0;
        let mut times = 0;
        loop {
            while n > 0 {
                result += (n % 10) * (n % 10);
                n /= 10;
            }
            if result == 1 || times > 6 {
                break;
            }
            times += 1;
            n = result;
            result = 0;
        }
        result == 1
    }
}