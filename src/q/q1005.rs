use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn largest_sum_after_k_negations(a: Vec<i32>, k: i32) -> i32 {

        // 方法1
        // let mut a = a;
        // let mut k = k;
        // a.sort();
        // let mut last_neg_idx = 0;
        //
        // for i in 0..a.len() {
        //     if k > 0 {
        //         if a[i] < 0 {
        //             last_neg_idx = i;
        //             a[i] = a[i].abs();
        //             k -= 1;
        //         } else {
        //             if k % 2 != 0 {
        //                 if a[i] > a[last_neg_idx] {
        //                     a[last_neg_idx] = -a[last_neg_idx];
        //                 } else {
        //                     a[i] = -a[i];
        //                 }
        //                 break;
        //             }
        //         }
        //     }
        // }
        //
        // a.iter().sum()

        // 方法2
        let mut bag = vec![0; 201];
        for i in 0..a.len() {
            bag[(100 + a[i]) as usize] += 1;
        }

        for _ in 0..k {
            for i in 0..bag.len() {
                if bag[i] > 0 {
                    bag[200 - i] += 1;
                    bag[i] -= 1;
                    break;
                }
            }
        }

        let mut sum = 0;
        for i in 0..bag.len() {
            if bag[i] > 0 {
                sum += (i as i32 - 100) * bag[i];
            }
        }
        sum
    }
}