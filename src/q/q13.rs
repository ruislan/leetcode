use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        // 方法1
        // let iter: Vec<char> = s.chars().collect();
        // let mut result = 0;
        // let len = iter.len();
        // let mut i = 0;
        // while i < len {
        //     let word = *iter.get(i).unwrap();
        //     if 'I' == word {
        //         if i + 1 != len {
        //             let sword = *iter.get(i + 1).unwrap();
        //             if sword == 'V' {
        //                 result += 4;
        //                 i += 1;
        //             } else if sword == 'X' {
        //                 result += 9;
        //                 i += 1;
        //             } else {
        //                 result += 1;
        //             }
        //         } else {
        //             result += 1;
        //         }
        //     } else if 'X' == word {
        //         if i + 1 != len {
        //             let sword = *iter.get(i + 1).unwrap();
        //             if sword == 'L' {
        //                 result += 40;
        //                 i += 1;
        //             } else if sword == 'C' {
        //                 result += 90;
        //                 i += 1;
        //             } else {
        //                 result += 10;
        //             }
        //         } else {
        //             result += 10;
        //         }
        //     } else if 'C' == word {
        //         if i + 1 != len {
        //             let sword = *iter.get(i + 1).unwrap();
        //             if sword == 'D' {
        //                 result += 400;
        //                 i += 1;
        //             } else if sword == 'M' {
        //                 result += 900;
        //                 i += 1;
        //             } else {
        //                 result += 100;
        //             }
        //         } else {
        //             result += 100;
        //         }
        //     } else if 'V' == word {
        //         result += 5;
        //     } else if 'L' == word {
        //         result += 50;
        //     } else if 'D' == word {
        //         result += 500;
        //     } else if 'M' == word {
        //         result += 1000;
        //     } else {
        //         panic!("wrong romans {}", s);
        //     }
        //     i += 1;
        // }
        // result

        // 方法2
        // let mut map = std::collections::HashMap::with_capacity(7);
        // map.insert('I', 1);
        // map.insert('V', 5);
        // map.insert('X', 10);
        // map.insert('L', 50);
        // map.insert('C', 100);
        // map.insert('D', 500);
        // map.insert('M', 1000);
        // let words:Vec<char> = s.chars().collect();
        // let len = words.len();
        // let mut result = 0;
        // let mut i = 0;
        // while i < len {
        //     let v = map.get(words.get(i).unwrap()).unwrap();
        //     if i < len - 1 && v < map.get(words.get(i + 1).unwrap()).unwrap() {
        //         result -= v;
        //     } else {
        //         result += v;
        //     }
        //     i += 1;
        // }
        // result

        // 方法3
        let mut map = std::collections::HashMap::new();
        map.insert('I', 1);
        map.insert('V', 5);
        map.insert('X', 10);
        map.insert('L', 50);
        map.insert('C', 100);
        map.insert('D', 500);
        map.insert('M', 1000);
        let mut result = 0;
        let mut last = 0;
        for word in s.chars() {
            let v = map.get(&word).unwrap();
            if last != 0 {
                let x = v / last;
                if x == 5 || x == 10 {
                    result -= last * 2;
                }
            }
            result += *v;
            last = *v;
        }
        result
    }
}