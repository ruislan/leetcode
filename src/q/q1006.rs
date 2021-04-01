use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn clumsy(n: i32) -> i32 {
        // 方法1
        // 前面4个数字都是正的
        // 从第5个数字开始到第8个数字看做整体，
        // 即 - (n5 * n6 / n7 - n8) ,注意是n7 - n8，而不是n7 + n8
        // 因为这里前面有-号了，所以要取反方向的
        // AC 0ms 2mb
        // fn cal(mut n: i32) -> i32 {
        //     let mut sum = n;
        //     n -= 1;
        //     if n > 0 { sum *= n; }
        //     n -= 1;
        //     if n > 0 { sum /= n; }
        //     n -= 1;
        //     if n > 0 { sum -= n; }
        //     n -= 1;
        //     let x = sum + if n > 0 { -cal(n) } else { 0 };
        //     -x
        // }
        // let mut n = n;
        // let mut sum = n;
        // n -= 1;
        // if n > 0 { sum *= n; }
        // n -= 1;
        // if n > 0 { sum /= n; }
        // n -= 1;
        // if n > 0 { sum += n; }
        // n -= 1;
        // sum + if n > 0 { cal(n) } else { 0 }   

        // 方法2
        // 优化一下方法1
        // AC 0ms 2mb
        fn cal(n: i32) -> i32 {
            if n <= 0 { return 0; }
            let mut sum = if n - 1 > 0 { n - 1 } else { 0 };
            sum *= if n - 2 > 0 { n - 2 } else { 1 };
            sum /= if n - 3 > 0 { n - 3 } else { 1 };
            n - sum + cal(n - 4)
        }
        let mut sum = n;
        sum *= if n - 1 > 0 { n - 1 } else { 1 };
        sum /= if n - 2 > 0 { n - 2 } else { 1 };
        sum + cal(n - 3)
    }
}