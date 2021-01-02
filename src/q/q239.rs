use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        // 方法1
        // 暴力法
        // 直接滑动窗口，然后计算窗口内的最大值就行了
        // 时间复杂度O(n*k) 额外空间复杂度：O(n)
        // let mut answer = Vec::new();
        // let k = k as usize;
        // for i in 0..=nums.len() - k {
        //     answer.push((i..i + k).map(|j| nums[j]).max().unwrap());
        // }
        // answer

        // 方法2
        // 用双端队列来记录最大值和它的索引
        // 不管窗口是多大的，当逐渐将值放入队列的时候
        // 遵循比前面一个小的话，就加入，
        // 比前面一个大的话，就弹出前面的
        // 然后如果迭代的索引已经超过了第一个索引，直接将它从前面移除
        // 例如；1,3,-1,-3,5,3,6,7; k = 3
        // 1 ->  i = 0, 空队列，直接加入[(1,0)]
        // 3 ->  i = 1,  3 > 1，弹出1，加入[(3,1)]
        // -1 -> i = 2, -1 < 3，加入[(3,1), (-1,2)]，结果为[3]
        // -3 -> i = 3, -3 < -1，加入[(3,1),(-1,2),(-3,3)]，结果为[3,3]
        // 5 ->  i = 4, i - k = 1, 移除头部1， 5 > -3,弹出-3; 5 > -1，弹出-1；空队列，加入[(5,4)],结果为[3,3,5]
        // 6 ->  i = 5, i - k = 2, 头部为4，   6 > 5，弹出5；空队列，加入[(6，5)]，结果为[3,3,5,6]
        // 7 ->  i = 6， i - k = 3，头部为5，   7 > 6，弹出6；空队列，加入[(7,6)]，结果为[3,3,5,6,7]
        // Passed 80ms 3.3mb
        let mut answer = Vec::new();
        let k = k as usize;
        let mut que = std::collections::VecDeque::new();
        que.push_back(0);
        for i in 1..nums.len() {
            if *que.front().unwrap() + k <= i { que.pop_front(); }
            while let Some(&back) = que.back() {
                if nums[i] <= nums[back] { break; }
                que.pop_back();
            }
            que.push_back(i);
            if i >= k - 1 {
                answer.push(nums[*que.front().unwrap()]);
            }
        }
        answer
    }
}