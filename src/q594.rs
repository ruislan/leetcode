mod q594 {
    pub fn find_lhs(nums: Vec<i32>) -> i32 {
        // 方法1
        // let mut sum = 0;
        // for i in 0..nums.len() {
        //     let mut count_max = 0;
        //     let mut count_min = 0;
        //     let mut count_me = 0;
        //     let max = nums[i] + 1;
        //     let min = nums[i] - 1;
        //     for j in 0..nums.len() {
        //         if nums[j] == nums[i] { count_me += 1;}
        //         if nums[j] == max { count_max += 1;}
        //         if nums[j] == min { count_min += 1;}
        //     }
        //     let count_max = if count_max > 0 { count_me + count_max } else { 0 };
        //     let count_min = if count_min > 0 { count_me + count_min } else { 0 };
        //     sum = std::cmp::max(sum, std::cmp::max(count_max,count_min));
        // }
        // sum

        // 方法2
        // if nums.len() < 1 { return 0; }
        // let mut sum = 0i32;
        // let mut map = std::collections::BTreeMap::new();
        // for i in 0..nums.len() {
        //     *map.entry(nums[i]).or_insert(0i32) += 1i32;
        // }
        // let mut last: Option<(i32, i32)> = None;
        // for (k, v) in map {
        //     if last != None {
        //         let num = last.unwrap();
        //         if k - num.0 == 1 && v + num.1 > sum {
        //             sum = v + num.1;
        //         }
        //     }
        //     last = Some((k, v));
        // }
        // sum

        // 方法3
        // let mut map = std::collections::HashMap::new();
        // let mut sum = 0;
        // for num in nums {
        //     *map.entry(num).or_insert(0) += 1;
        //     if map.contains_key(&(num + 1)) {
        //         sum = std::cmp::max(sum, map.get(&(num)).unwrap() + map.get(&(num + 1)).unwrap());
        //     }
        //     if map.contains_key(&(num - 1)) {
        //         sum = std::cmp::max(sum, map.get(&(num)).unwrap() + map.get(&(num - 1)).unwrap());
        //     }
        // }
        // sum

        // 方法4
        if nums.len() < 1 { return 0; }
        let mut nums = nums;
        nums.sort();
        let mut v = Vec::new();
        let mut num_ptr = 0;
        let mut last = None;
        for i in 0..nums.len() {
            if last == None {
                last = Some(nums[i]);
                v.push((nums[i], 1));
            } else if last == Some(nums[i]) {
                v[num_ptr].1 += 1;
            } else {
                last = Some(nums[i]);
                num_ptr += 1;
                v.push((nums[i], 1));
            }
        }

        let mut sum = 0;
        let mut last = v[0];
        for i in 1..v.len() {
            if v[i].0 - last.0 == 1 && v[i].1 + last.1 > sum {
                sum = v[i].1 + last.1;
            }
            last = v[i];
        }
        sum
    }
}