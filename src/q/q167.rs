use crate::q::Solution;

#[allow(unused)]
impl Solution {
    // 方法名two_sum与q1相同，更名为two_sum_ii
    pub fn two_sum_ii(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        // 方法1
        // for i in 0..numbers.len() {
        //     for j in (i + 1)..numbers.len() {
        //         if numbers[i] + numbers[j] == target {
        //             return vec![i as i32 + 1, j as i32 + 1]
        //         }
        //     }
        // }
        // vec![]

        // 方法2
        // let mut len = numbers.len();
        // for i in 0..len {
        //     for j in i + 1..len {
        //         if numbers[i] + numbers[j] == target {
        //             return vec![i as i32 + 1, j as i32 + 1];
        //         }
        //         if numbers[i] + numbers[j] > target {
        //             break;
        //         }
        //     }
        // }
        // vec![]

        // 方法3
        let mut left = 0;
        let mut right = numbers.len() - 1;
        while left < right {
            let sum = numbers[left] + numbers[right];
            if sum > target {
                right -= 1;
            } else if sum < target {
                left += 1;
            } else {
                return vec![left as i32 + 1, right as i32 + 1];
            }
        }
        vec![]
    }
}