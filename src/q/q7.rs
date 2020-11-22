use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn reverse(x: i32) -> i32 {
        // 方法1
        // let mut rev = 0;
        // while x != 0 {
        //     let pop = x % 10;
        //     x /= 10;
        //     if (rev > (i32::max_value() / 10)) || (rev == (i32::max_value() / 10) && pop > 7) {
        //         return 0;
        //     }
        //     if (rev < (i32::min_value() / 10)) || (rev == (i32::min_value() / 10) && pop < -8) {
        //         return 0;
        //     }
        //     rev = rev * 10 + pop;
        // }
        // rev

        // 方法2
        if x == 0 { return 0; }

        let rev =
            if x > 0 {
                let rev_str = x.to_string().chars().rev().collect::<String>();
                rev_str.parse::<i64>().unwrap()
            } else {
                let mut rev_str = x.to_string();
                rev_str.remove(0);
                let mut rev_str = rev_str.chars().rev().collect::<String>();
                rev_str = String::from("-") + rev_str.as_str();
                rev_str.parse::<i64>().unwrap()
            };

        if rev > i32::max_value() as i64 || rev < i32::min_value() as i64 {
            0
        } else {
            rev as i32
        }
    }
}