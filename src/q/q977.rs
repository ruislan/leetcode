use crate::q::Solution;

impl Solution {
    pub fn sorted_squares(a: Vec<i32>) -> Vec<i32> {
        // 方法1
        // let mut t = 0;
        // for i in 0..a.len() {
        //     if a[i] >= 0 {
        //         t = i;
        //         break;
        //     }
        // }
        //
        // let mut left = t as i32 - 1;
        // let mut right = t;
        // let mut res = Vec::new();
        // while right < a.len() && left >= 0 {
        //     if a[left as usize].abs() < a[right].abs() {
        //         res.push(a[left as usize] * a[left as usize]);
        //         left -= 1;
        //     } else {
        //         res.push(a[right] * a[right]);
        //         right += 1;
        //     }
        // }
        //
        // while left >= 0 {
        //     res.push(a[left as usize] * a[left as usize]);
        //     left -= 1;
        // }
        //
        // while right < a.len() {
        //     res.push(a[right] * a[right]);
        //     right += 1;
        // }
        // res
        // 方法2
        let mut bag = vec![0; 10001];
        for i in 0..a.len() {
            bag[a[i].abs() as usize] += 1;
        }
        let mut res = Vec::new();
        for i in 0..bag.len() {
            if bag[i] > 0 {
                for _ in 0..bag[i] {
                    res.push(i.pow(2) as i32);
                }
            }
        }
        res
    }
}

#[test]
fn test_q977() {
    assert_eq!(
        vec![0, 1, 9, 16, 100],
        Solution::sorted_squares(vec![-4_i32, -1_i32, 0_i32, 3_i32, 10_i32])
    );
}