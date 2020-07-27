use crate::offer::Solution;

impl Solution {
    pub fn find_repeat_number(nums: Vec<i32>) -> i32 {
        // 方法1
        // 利用hashmap存储数字，迭代nums，一旦出现有重复的，立刻返回
        // Passed 8ms 2.1mb
        // let mut map = std::collections::HashSet::new();
        // *nums.iter().find(|x| !map.insert(**x)).unwrap_or(&nums[0])
        let mut map = std::collections::HashSet::new();
        for &x in &nums {
            if !map.insert(x) { return x; }
        }
        nums[0]

        // 方法2
        // 换成vec来存储数字
        // Passed 4ms 3.2mb
        // let mut arr = vec![0; 100001];
        // *nums.iter().find(|&&x| {
        //     arr[x as usize] += 1;
        //     arr[x as usize] > 1
        // }).unwrap_or(&nums[0])
        // 换成for试试
        // Passed 4ms 3.1mb
        // let mut arr = vec![0; 100001];
        // for &x in &nums {
        //     let x = x as usize;
        //     arr[x] += 1;
        //     if arr[x] > 1 { return x as i32; }
        // }
        // nums[0]
    }
}

#[test]
fn test() {
    assert_eq!(Solution::find_repeat_number(vec![2, 3, 1, 0, 2, 5, 3]), 2);
    assert_eq!(Solution::find_repeat_number(vec![2, 2]), 2);
    assert_eq!(Solution::find_repeat_number(vec![2, 3]), 2);
}