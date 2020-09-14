use crate::q::Solution;

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        // 方法1

        // fn reverse(nums: &mut Vec<i32>, mut from: usize, mut to: usize) {
        //     while from < to {
        //         nums.swap(from, to);
        //         from += 1;
        //         to -= 1;
        //     }
        // }
        
        // let len = nums.len();
        // let k = k as usize;
        // let k = if k > len { k - len } else { k };
        // if k == 0 { return; }
        // Solution::reverse(nums, 0, len - 1);
        // Solution::reverse(nums, 0, k - 1);
        // Solution::reverse(nums, k, len - 1);

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
                std::mem::swap(&mut nums[next], &mut prev);
                current = next;
                count += 1;
                if start == current { break; }
            }
            start += 1;
        }
    }
}