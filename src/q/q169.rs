use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        // 方法1
        // let mut map = std::collections::HashMap::new();
        // for num in nums {
        //     let counter = map.entry(num).or_insert(0);
        //     *counter += 1;
        // }
        // let mut num = 0i32;
        // let mut num_count = 0i32;
        // for (k,v) in map.iter() {
        //     if *v > num_count {
        //         num = *k;
        //         num_count = *v;
        //     }
        // }
        // num

        // 方法2
        // nums.sort();
        // let threshold = nums.len() as i32 / 2i32;
        // let mut max_num = None;
        // let mut num_count = 0i32;
        // for num in nums {
        //     if max_num == None || num != max_num.unwrap() { max_num = Some(num); }
        //     if num == max_num.unwrap() { num_count += 1; }
        //     if num_count > threshold { return num; }
        // }
        // max_num.unwrap()

        // 方法3
        // nums.sort();
        // nums[nums.len() / 2]

        // 方法4
        // for num in &nums {
        //     let mut count = 0;
        //     for n in &nums {
        //         if *n == *num { count += 1; }
        //     }
        //     if count > nums.len() / 2 { return *num; }
        // }
        // 0
        // 方法5
        
        let mut max_num = 0i32;
        let mut count = 0i32;
        for num in nums {
            if count == 0 { max_num = num; }
            if max_num == num { count += 1; } else { count -= 1; }
        }
        max_num
    }
}