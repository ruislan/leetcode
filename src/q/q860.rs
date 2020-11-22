use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn lemonade_change(bills: Vec<i32>) -> bool {
        let (mut d10, mut d5) = (0_i32, 0_i32);
        for i in 0..bills.len() {
            match bills[i] {
                20 => {
                    if d10 > 0 && d5 > 0 {
                        // d20 += 1;
                        d10 -= 1;
                        d5 -= 1;
                    } else if d10 == 0 && d5 >= 3 {
                        // d20 += 1;
                        d5 -= 3;
                    } else {
                        return false;
                    }
                }
                10 => {
                    if d5 >= 1 {
                        d10 += 1;
                        d5 -= 1;
                    } else {
                        return false;
                    }
                }
                _ => {
                    d5 += 1;
                }
            }
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
