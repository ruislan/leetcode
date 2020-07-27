mod q_189 {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        // 方法1
        // let len = nums.len();
        // let k = k as usize % len;
        // reverse(nums, 0, len - 1);
        // reverse(nums, 0, k);
        // reverse(nums, k, len - k);

        // 方法2
        // let mut count = 0;
        // while count < k {
        //     let num = nums.pop().unwrap();
        //     nums.insert(0, num);
        //     count += 1;
        // }

        // 方法3
        // let len = nums.len();
        // let k = if k > len as i32 { k - len as i32 } else { k };
        // let mut left = nums.drain(0..(len - k as usize)).collect();
        // nums.append(&mut left)

        // 方法4
        let k = k as usize % nums.len();
        let mut count = 0;
        let mut start = 0;
        while count < nums.len() {
            let mut current = start;
            let mut prev = nums[start];
            loop {
                let next = (current + k) % nums.len();
                let temp = nums[next];
                nums[next] = prev;
                prev = temp;
                current = next;
                count += 1;
                if start == current { break; }
            }
            start += 1;
        }
    }

    fn reverse(nums: &mut Vec<i32>, from: usize, to: usize) {
        while from <= to {
            let temp = nums[from];
            nums[from] = nums[to];
            nums[to] = temp;
        }
    }
}