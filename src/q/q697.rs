use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn find_shortest_sub_array(nums: Vec<i32>) -> i32 {
        // 方法1
        // HashMap求出每个数字的频率和起止点
        // 然后找出最大的那个频率（也就是度）的数字
        // 最后将HashMap中频率为这个数字的所有的起止点都拿出来求出距离
        // 距离最小的那个就是结果
        // AC 4ms 2.6mb
        // let mut map = std::collections::HashMap::new();
        // for i in 0..nums.len() {
        //     let mut e = map.entry(nums[i]).or_insert((0, i, 0));
        //     e.0 += 1;
        //     e.2 = i;
        // }
        //
        // let mut max = 0;
        // for (_, v) in &map {
        //     if v.0 > max { max = v.0; }
        // }
        //
        // let mut min_len = 0;
        // for (_, v) in &map {
        //     if v.0 == max {
        //         let cur_len = v.2 - v.1 + 1;
        //         if min_len == 0 || cur_len < min_len { min_len = cur_len; }
        //     }
        // }
        // min_len as i32

        // 方法2
        // 今天的每日一题，这次用滑动窗口来解决
        // 先求出数组的度（degree）
        // 然后移动窗口，当窗口中的数字的频率正好等于度时
        // 我们收缩窗口到保持这个度的最小，然后求出距离
        // 所有的距离中最小的就是结果
        // AC 4ms 2.3mb
        let mut freq = std::collections::HashMap::new();
        for &x in nums.iter() { *freq.entry(x).or_insert(0) += 1; }
        let degree = freq.into_iter().map(|(k, v)| v).max().unwrap();
        let n = nums.len();
        let mut answer = n + 1;
        let mut window_freq = std::collections::HashMap::new();
        let mut lo = 0;
        for hi in 0..n {
            let count = window_freq.entry(nums[hi]).or_insert(0);
            *count += 1;
            if *count < degree { continue; }
            while nums[lo] != nums[hi] {
                *window_freq.get_mut(&nums[lo]).unwrap() -= 1;
                lo += 1;
            }
            answer = answer.min(hi - lo + 1);
        }
        answer as i32
    }
}