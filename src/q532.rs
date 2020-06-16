mod q532 {
    pub fn find_pairs(nums: Vec<i32>, k: i32) -> i32 {
        // 方法1
        // let mut kv = std::collections::HashMap::new();
        // for i in 0..nums.len() {
        //     for j in i+1..nums.len() {
        //         if (nums[i] - nums[j]).abs() == k {
        //             let e = if nums[i] > nums[j] { (nums[j], nums[i]) } else { (nums[i], nums[j] )};
        //             kv.entry(e.0).or_insert(e.1);
        //         }
        //     }
        // }
        // kv.len() as i32
        
        // 方法2
        if k < 0 { return 0; }
        let mut saw = std::collections::HashSet::new();
        let mut diff = std::collections::HashSet::new();
        for num in nums {
            if saw.contains(&(num - k)) {
                diff.insert(num - k);
            }
            if saw.contains(&(num + k)) {
                diff.insert(num);
            }
            saw.insert(num);
        }
        diff.len() as i32
    }
}