mod q367 {
    pub fn is_perfect_square(num: i32) -> bool {
        let mut left = 0;
        let mut right = num;
        while left <= right {
            let mid = (left + right) / 2;
            let sqrt: i64 = mid as i64 * mid as i64;
            if sqrt == num as i64 { return true; } else if sqrt > num as i64 {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }
        false
    }
}