mod q496 {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        // 方法1
        // let mut res = Vec::new();
        // for i in 0..nums1.len() {
        //     let mut found = false;
        //     for j in 0..nums2.len() {
        //         if nums1[i] == nums2[j] {
        //             for k in j + 1..nums2.len() {
        //                 if nums2[k] > nums1[i] {
        //                     res.push(nums2[k]);
        //                     found = true;
        //                     break;
        //                 }
        //             }
        //         }
        //     }
        //     if !found {
        //         res.push(-1);
        //     }
        // }
        // res
        
        // 方法2
        let mut res = Vec::new();
        let mut stack = Vec::new();
        let mut map = std::collections::HashMap::new();
        for i in 0..nums2.len() {
            while !stack.is_empty() && nums2[i] > *stack.last().unwrap() {
                map.insert(stack.pop().unwrap(), nums2[i]);
            }
            stack.push(nums2[i]);
        }
        while let Some(n) = stack.pop() {
            map.insert(n, -1);
        }
        for n1 in nums1 {
            res.push(*map.get(&n1).unwrap());
        }
        res
    }
}