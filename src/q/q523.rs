use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn check_subarray_sum(mut nums: Vec<i32>, k: i32) -> bool {
        // 方法1
        // 暴力求解
        // TLE
        // let n = nums.len();
        // for i in 0..n - 1 {
        //     let mut sum = nums[i];
        //     for j in i+1..n {
        //         sum += nums[j];
        //         if sum % k == 0 { return true; }
        //     }
        // }
        // false

        // 方法2
        // 最开始以为是滑动窗口，但是发现这里有问题的就是n倍都可以，不单单是k。
        // 其实这里我们要的就是 sum[i..j]这个区域的值是不是k的n倍就行了
        // 求解区间和的话，前缀和是最容易想到的，但只是前缀和还是O(n^2)，因为要循环两次
        // 所以，这里需要优化，那么我们说如果一个数字是k的n倍，那么必然有a % k == b % k
        // 也就是两个数字同余，在这里就是两个和是同余数就可以了
        // 所以我们用一个map来装余数和它上一次的位置，只要我们的前缀和有一个余数出现过，那么就是真
        // 这里还要注意的一个地方就是，必须是两个连续，所以我们还要判断一下两个的位置是不是大于1
        // 这里还有一个坑，就是k可以取值为0
        // AC 20ms 4.5mb
        let n = nums.len();
        let mut prefix_sum = 0;
        let mut map = std::collections::HashMap::new();
        map.insert(0, -1);
        for i in 0..n {
            prefix_sum += nums[i];
            let rem = if k == 0 { prefix_sum } else { prefix_sum % k };
            if map.contains_key(&rem) {
                if i as i32 - map[&rem] > 1 { return true; }
            } else {
                map.insert(rem, i as i32);
            }
        }
        false
    }
}