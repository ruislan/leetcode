mod q908 {
    pub fn smallest_range_i(a: Vec<i32>, k: i32) -> i32 {
        // 方法1
        // let mut a = a;
        // a.sort();
        // let x1 = a[a.len() - 1] - k;
        // let x2 = a[0] + k;
        // if x2 > x1 { 0 } else { x1 - x2 }

        // 方法2
        let (mut min, mut max) = (a[0], a[0]);
        for i in 0..a.len() {
            if a[i] > max { max = a[i]; }
            if a[i] < min { min = a[i]; }
        }
        let x1 = max - k;
        let x2 = min + k;
        if x2 > x1 { 0 } else { x1 - x2 }
    }
}