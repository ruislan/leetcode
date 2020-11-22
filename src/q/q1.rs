use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // 方法1
        // let mut nums_hash:HashMap<i32, i32> = HashMap::new();
        // let mut i:i32 = 0;
        // loop {
        //     let n = nums.get(i as usize).unwrap();
        //     let num = target - n;
        //     let num_index = nums_hash.get(&num);
        //     if None != num_index {
        //         return vec![*num_index.unwrap(), i];
        //     } else {
        //         nums_hash.insert(*n, i);
        //     }
        //     i += 1;
        //     if i as usize > nums.len() {
        //         break;
        //     }
        // }
        // return Vec::new();

        // 方法2
        // let mut nums_hash:HashMap<i32, i32> = HashMap::new();
        // for i in 0..nums.len() {
        //     let n = nums.get(i).unwrap();
        //     let num = target - n;
        //     let num_index = nums_hash.get(&num);
        //     if None != num_index {
        //         return vec![*num_index.unwrap(), i as i32];
        //     } else {
        //         nums_hash.insert(*n, i as i32);
        //     }
        // }
        // return Vec::new();

        // 方法3
        // let len = nums.len();
        // for i in 0..len {
        //     let n1 = nums[i];
        //     for k in (i + 1)..len {
        //         let n2 = nums[k];
        //         if (n1 + n2) == target {
        //             return vec![i as i32, k as i32];
        //         }
        //     }
        // }
        // return Vec::new();

        // 方法4
        let mut nums_hash = std::collections::HashMap::new();
        for i in 0..nums.len() {
            let n = nums[i];
            let num = target - n;
            let num_index = nums_hash.get(&num);
            if None != num_index {
                return vec![*num_index.unwrap(), i as i32];
            } else {
                nums_hash.insert(n, i as i32);
            }
        }
        return Vec::new();
    }
}