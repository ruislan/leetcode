mod q905 {
    pub fn sort_array_by_parity(a: Vec<i32>) -> Vec<i32> {
        let mut a = a;
        let (mut left, mut right) = (0, a.len() - 1);
        while left < right {
            if a[left] % 2 == 0 {
                left += 1;
                continue;
            }
            if a[right] % 2 == 1 {
                right -= 1;
                continue;
            }
            a.swap(left, right);
            left += 1;
            right -= 1;
        }
        a
    }
}