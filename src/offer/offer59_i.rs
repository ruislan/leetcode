use crate::offer::Solution;

// q239的简单版本
// q239是一个“困难”题，线性时间内解决的方法在q239里面去思考了
// 这里是“简单”题，就用简单的方式处理
impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        // 方法1
        // 利用库函数windows来滑动窗口，并求出其中的最大值，实际上这个算法o(n*n)
        // Passed 20ms 2.6mb
        // match nums.len() {
        //     0 => vec![],
        //     _ => nums.windows(k as usize).map(|x| *x.iter().max().unwrap()).collect()
        // }

        // 方法2
        // 我们尾部压入，头部弹出，有点先进先出的意思，这里我们可以使用一个双端队列来处理，
        // 避免使用数组的操作，因为数组操作会移位
        // Passed 20ms 2.6mb
        let (k, mut queue, mut res) = (k as usize, std::collections::VecDeque::new(), Vec::new());
        for i in 0..k { queue.push_back(nums[i]); }
        if !queue.is_empty() { res.push(*queue.iter().max().unwrap()); }
        for i in k..nums.len() {
            queue.pop_front();
            queue.push_back(nums[i]);
            res.push(*queue.iter().max().unwrap());
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(Solution::max_sliding_window(vec![], 0), vec![]);
    assert_eq!(Solution::max_sliding_window(vec![1], 1), vec![1]);
    assert_eq!(Solution::max_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3), vec![3, 3, 5, 5, 6, 7]);
}