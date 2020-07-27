mod q217 {
    pub fn contains_duplicate(mut nums: Vec<i32>) -> bool {
        // 方法1
        // for i in 0..nums.len() {
        //     let n1 = nums[i];
        //     for j in (i + 1)..nums.len() {
        //         let n2 = nums[j];
        //         if n1 == n2 { return true; }
        //     }
        // }
        // false

        // 方法2
        // let len = nums.len();
        // nums.sort();
        // nums.dedup();
        // len != nums.len()

        // 方法3
        // let mut map = std::collections::HashMap::new();
        // for num in nums {
        //     let count = map.entry(num).or_insert(0);
        //     *count += 1;
        //     if *count >= 2 { return true; }
        // }
        // false

        // 方法4
        // let mut sets = std::collections::HashSet::new();
        // for num in nums {
        //     if sets.contains(&num) { return true; }
        //     sets.insert(num);
        // }
        // false

        // 方法5
        nums.sort();
        for i in 1..nums.len() {
            if nums[i] == nums[i - 1] { return true; }
        }
        false
    }
}