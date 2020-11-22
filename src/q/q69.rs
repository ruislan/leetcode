use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn my_sqrt(n: i32) -> i32 {
        // 方法1
        // let mut res = 0;
        // for i in 1..=x {
        //     if i as i64 * i as i64 > x as i64 { break; }
        //     res = i;
        // }
        // res as i32

        // 方法2
        // let mut res = 0;
        // let mut left = 1;
        // let mut right = x;
        // while left <= right {
        //     let mid = left + (right - left) / 2;
        //     let q: i64 = mid as i64 * mid as i64;
        //     if q == x as i64 { return mid as i32; } else if q > x as i64 { right = mid - 1; } else {
        //         left = mid + 1;
        //         res = mid;
        //     }
        // }
        // res

        // 方法3
        // (x as f64).sqrt() as i32

        // 方法4
        // let mut res = 0;
        // let x = x as i64;
        // for i in 1..=(x+1)/2 {
        //     if i as i64 * i as i64 > x as i64 { break; }
        //     else { res = i }
        // }
        // res as i32

        // 方法5
        let mut x = 1f64;
        loop {
            let fx = (x + n as f64 / x) / 2f64;
            if (x - fx).abs() < 0.00001f64 { break; }
            x = fx;
        }
        x as i32
    }
}