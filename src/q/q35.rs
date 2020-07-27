mod q35 {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        // 方法1
        // nums.binary_search(&target).unwrap_or_else(|x| x) as i32
        
        // 方法2
        let mut left = 0;
        let mut right = nums.len() - 1;

        while left <= right {
            let mid = (left + right) / 2;
            if target == nums[mid] {
                return mid as i32;
            } else if target > nums[mid] {
                left = mid + 1;
            } else {
                if mid == 0 { break; }
                right = mid - 1;
            }
        }
        left as i32
    }
}