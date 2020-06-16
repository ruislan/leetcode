mod q1 {
    use std::collections::HashMap;

    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut nums_hash: HashMap<i32, i32> = HashMap::new();
        for i in 0..nums.len() {
            let n = nums[i];
            let num = target - n;
            let num_index = nums_hash.get(&num);
            if None != num_index {
                return vec![*num_index.unwrap(), i as i32];
            } else {
                nums_hash.insert(n, i as i32);
            }
        } // let mut i:i32 = 0;
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
        return Vec::new();
    }
}