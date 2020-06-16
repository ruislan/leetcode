mod q11 {
    pub fn max_area(height: Vec<i32>) -> i32 {
        // 方法1
        // Passed 552ms 2mb
        // let mut max = 0;
        // for i in 0..height.len() {
        //     let n = height[i];
        //     for j in 1..height.len() {
        //         let m = height[j];
        //         if m > n {
        //             if n * (j - i) as i32 > max { max = n * (j - i) as i32; }
        //         } else {
        //             if m * (j - i) as i32 > max { max = m * (j - i) as i32; }
        //         }
        //     }
        // }
        // max

        // 方法2
        // Passed 0ms 2.2mb
        let mut max = 0;
        let (mut i, mut j) = (0, height.len() - 1);
        while i < j {
            let n_left = height[i];
            let n_right = height[j];
            if n_left > n_right {
                if max < n_right * (j - i) as i32 {
                    max = n_right * (j - i) as i32;
                }
                j -= 1;
            } else {
                if max < n_left * (j - i) as i32 {
                    max = n_left * (j - i) as i32;
                }
                i += 1;
            }
        }
        max
    }
}