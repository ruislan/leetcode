mod q136 {
    pub fn single_number(mut nums: Vec<i32>) -> i32 {
        // 方法1
        // let mut map = std::collections::HashMap::new();
        // while let Some(num) = nums.pop() {
        //     let n = map.get(&num);
        //     if n == None { map.insert(num, num); }
        //     else { map.remove(&num); }
        // };
        // *map.keys().next().unwrap_or(&0)

        // 方法2
        // let mut sets = std::collections::HashSet::new();
        // let mut total = 0i32;
        // for num in nums {
        //     total += num;
        //     sets.insert(num);
        // }
        // 2 * sets.iter().sum::<i32>() - total

        // 方法3
        // nums.sort();
        // let mut i = 0;
        // while i < nums.len() - 1 {
        //     if nums[i] != nums[i + 1] { return nums[i]; } else { i += 2; }
        // }
        // nums[nums.len() - 1]

        // 方法4
        let mut result = 0;
        for num in nums {
            result = result ^ num;
        }
        result
    }
}