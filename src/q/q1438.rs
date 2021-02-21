use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn longest_subarray(nums: Vec<i32>, limit: i32) -> i32 {
        // 方法1
        // 暴力
        // 双层迭代，找出最大绝对差符合条件的最大的即可
        // 一般来说会超时，就不用这个提交了
        // let n = nums.len();
        // let mut answer = 0;
        // for i in 0..n {
        //     let mut max = nums[i];
        //     let mut min = nums[i];
        //     for j in i + 1..n {
        //         max = max.max(nums[j]);
        //         min = min.min(nums[j]);
        //         if max - min <= limit {
        //             answer = answer.max(j - i + 1)
        //         }
        //     }
        // }
        // answer as i32

        // 方法2
        // 滑动窗口
        // 利用大顶堆和小顶堆来存储两边的值
        // 这里有个地方要注意
        // 我们在添加当前数字到最大最小堆之前，需要先去除掉堆中无用的数据
        // 这里无用的数据就是，如果当前的数字大于大堆顶的数字，就要清空大顶堆
        // 如果当前的数字小于小堆顶的数字，就要清空小顶堆
        // AC 16ms 3.8mb
        // use std::collections::BinaryHeap;
        // use std::cmp::Reverse;
        //
        // let n = nums.len();
        // let mut lo = 0;
        // let mut answer = 0;
        // let mut smaller:BinaryHeap<Reverse<i32>> = BinaryHeap::new();
        // let mut greater:BinaryHeap<i32> = BinaryHeap::new();
        // for hi in 0..n {
        //     while !smaller.is_empty() && nums[hi] < smaller.peek().unwrap().0 {
        //         smaller.pop();
        //     }
        //     while !greater.is_empty() && nums[hi] > *greater.peek().unwrap() {
        //         greater.pop();
        //     }
        //     smaller.push(Reverse(nums[hi]));
        //     greater.push(nums[hi]);
        //     while !greater.is_empty() && !smaller.is_empty() && *greater.peek().unwrap() - smaller.peek().unwrap().0 > limit {
        //         if nums[lo] == *greater.peek().unwrap() { greater.pop(); }
        //         if nums[lo] == smaller.peek().unwrap().0 { smaller.pop(); }
        //         lo += 1;
        //     }
        //     answer = answer.max(hi + 1 - lo);
        // }
        // answer as i32

        // 方法3
        // 滑动窗口，中间数据结构使用BTreeMap
        // BTreeMap是一个排序的树，它能够维护数组内的最大和最小值，
        // 且能够很好的支持删除操作，这样一来，我们就不需要两个堆来维护这个结构了
        // AC 36ms 3mb
        let n = nums.len();
        let mut lo = 0;
        let mut answer = 0;
        let mut bts = std::collections::BTreeMap::new();
        for hi in 0..n {
            *bts.entry(nums[hi]).or_insert(0) += 1;
            while *bts.keys().last().unwrap() - *bts.keys().next().unwrap() > limit {
                let freq = bts.get_mut(&nums[lo]).unwrap();
                *freq -= 1;
                if *freq == 0 {
                    bts.remove(&nums[lo]);
                }
                lo += 1;
            }
            answer = answer.max(hi + 1 - lo);
        }
        answer as i32
    }
}