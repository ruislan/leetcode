mod q728 {
    pub fn self_dividing_numbers(left: i32, right: i32) -> Vec<i32> {
        let mut res = Vec::new();
        for i in left..=right {
            let mut num = i;
            let mut div = num;
            let mut good = true;
            while div > 0 {
                if div % 10 == 0 || num % (div % 10) != 0 {
                    good = false;
                    break;
                }
                div /= 10;
            }
            if good { res.push(i); }
        }
        res
    }
}