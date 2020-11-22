use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn rotated_digits(n: i32) -> i32 {
        let mut sum = 0;
        for num in 0..=n {
            let mut x = num;
            let mut new_x = 0;
            let mut c = 0;
            let mut good = true;
            while x > 0 {
                let mut y = x % 10;
                match y {
                    3 | 4 | 7 => good = false,
                    2 => y = 5,
                    5 => y = 2,
                    6 => y = 9,
                    9 => y = 6,
                    _ => y = y,
                }
                if !good { break; }

                new_x += y * 10i32.pow(c);
                x /= 10;
                c += 1;
            }
            if good && new_x != num { sum += 1; }
        }
        sum
    }
}