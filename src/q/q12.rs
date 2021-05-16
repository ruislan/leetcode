use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn int_to_roman(mut num: i32) -> String {
        // 方法1
        // 首先处理最大的数字，直到数字不够大之后
        // 再处理较小的数字，直到数字为0
        // AC 4ms 2mb
        // let mut result = String::new();
        // while num >= 1000 {
        //     result += "M";
        //     num -= 1000;
        // }
        // while num >= 900 {
        //     result += "CM";
        //     num -= 900;
        // }
        // while num >= 500 {
        //     result += "D";
        //     num -= 500;
        // }
        // while num >= 400 {
        //     result += "CD";
        //     num -= 400;
        // }
        // while num >= 100 {
        //     result += "C";
        //     num -= 100;
        // }
        // while num >= 90 {
        //     result += "XC";
        //     num -= 90;
        // }
        // while num >= 50 {
        //     result += "L";
        //     num -= 50;
        // }
        // while num >= 40 {
        //     result += "XL";
        //     num -= 40;
        // }
        // while num >= 10 {
        //     result += "X";
        //     num -= 10;
        // }
        // while num >= 9 {
        //     result += "IX";
        //     num -= 9;
        // }
        // while num >= 5 {
        //     result += "V";
        //     num -= 5;
        // }
        // while num >= 4 {
        //     result += "IV";
        //     num -= 4;
        // }
        // while num >= 1 {
        //     result += "I";
        //     num -= 1;
        // }
        // result

        // 方法2
        // 方法1太朴素，我们可以看出来其实可以优化成数组来处理
        // AC 8ms 2mb
        let mut answer = String::new();
        let mut num = num;
        let values = vec![1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];
        let romans = vec!["M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I"];
        for i in 0..13 {
            while num >= values[i] {
                answer += romans[i];
                num -= values[i];
            }
        }
        answer
    }
}