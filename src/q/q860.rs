use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn lemonade_change(bills: Vec<i32>) -> bool {
        // 方法1
        // 今天轮到这道题每日一题了，之前没有写解题思路，这里补上之前的思路
        // 20的时候：
        //    如果有5元和10元：减去5元和10元
        //    如果只有5元，且大于等于3个：减去3个5元
        //    否则就返回不能找零
        // 10元的时候：
        //    如果有5元：减去5元
        //    否则就返回不能找零
        // 5元的时候：
        //    增加5元就行了
        // 迭代结束，返回能找零
        // Passed 0ms 2.1mb
        // let (mut d10, mut d5) = (0_i32, 0_i32);
        // for i in 0..bills.len() {
        //     match bills[i] {
        //         20 => {
        //             if d10 > 0 && d5 > 0 {
        //                 // d20 += 1;
        //                 d10 -= 1;
        //                 d5 -= 1;
        //             } else if d10 == 0 && d5 >= 3 {
        //                 // d20 += 1;
        //                 d5 -= 3;
        //             } else {
        //                 return false;
        //             }
        //         }
        //         10 => {
        //             if d5 >= 1 {
        //                 d10 += 1;
        //                 d5 -= 1;
        //             } else {
        //                 return false;
        //             }
        //         }
        //         _ => {
        //             d5 += 1;
        //         }
        //     }
        // }
        // true

        // 方法2
        // 方法1的思路还算清晰，但是有点冗长
        // 这次每日一题，用kotlin做的时候，思路和方法1差不多，但是更加精炼
        // 如果遇到5元，我们5元加1
        // 如果遇到10元，我们减去一个5元
        // 如果遇到20元，有10元，我们先减去10元，再减去5元；没有10元，直接减去3个5元
        // 然后判断当前5元的数量是不是小于0了，如果是，那么就不能找零
        // 迭代结束，返回能找零
        // Passed 0ms 2.1mb
        let mut five = 0;
        let mut ten = 0;
        for bill in bills {
            match bill {
                5 => five += 1,
                10 => {
                    five -= 1;
                    ten += 1;
                }
                _ => {
                    if ten > 0 {
                        ten -= 1;
                        five -= 1;
                    } else {
                        five -= 3;
                    }
                }
            }
            if five < 0 { return false; }
        }
        true
    }
}

#[test]
fn test_q860() {
    assert_eq!(false, Solution::lemonade_change(vec![10]));
    assert_eq!(true, Solution::lemonade_change(vec![5]));
    assert_eq!(false, Solution::lemonade_change(vec![20]));
    assert_eq!(true, Solution::lemonade_change(vec![5, 5, 5, 10, 20]));
    assert_eq!(true, Solution::lemonade_change(vec![5, 5, 10]));
    assert_eq!(false, Solution::lemonade_change(vec![10, 10]));
    assert_eq!(false, Solution::lemonade_change(vec![5, 5, 10, 10, 20]));
}
