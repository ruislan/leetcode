use crate::q::Solution;

impl Solution {
    pub fn divisor_game(n: i32) -> bool {
        // 方法1
        // n & 1 == 0
        // 方法2
        // n % 2 == 0
        // 方法3
        let mut a = true;
        let mut n = n;
        while n > 0 {
            let mut x = 1;
            while x < n {
                if n % x == 0 { break; } else { x += 1; }
            }
            n -= x;
            if x != n {
                a = !a;
            } else {
                break;
            }
        }
        a
    }
}