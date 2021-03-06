use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
        // 方法1
        // 暴力解法，每次都循环找出下一个最大的数字
        // O(n^2)
        // AC 552ms 2mb
        // let n = nums.len();
        // let mut answer = vec![0; n];
        // for i in 0..n {
        //     let mut j = i + 1;
        //     j %= n;
        //     while j != i {
        //         if nums[i] < nums[j] { break; }
        //         j += 1;
        //         j %= n;
        //     }
        //     if j == i {
        //         answer[i] = -1;
        //     } else {
        //         answer[i] = nums[j];
        //     }
        // }
        // answer

        // 方法2
        // 依然是暴力法，但是略微优化了一下
        // 主要在于取余这个操作有点耗时，
        // 而且我们发现虽然是循环，但是其实不会循环几圈，最多循环1圈
        // 那么完全可以将数组重复一次，避免了取余操作
        // AC 48ms 2.1mb
        // 可以看到，虽然还是O(n^2)，但是进步巨大
        // let n = nums.len();
        // let nums = nums.repeat(2);
        // let mut answer = vec![0; n];
        // for i in 0..n {
        //     let mut j = i + 1;
        //     while j < n * 2 && nums[i] >= nums[j] {
        //         j += 1;
        //     }
        //     if j == n * 2 {
        //         answer[i] = -1;
        //     } else {
        //         answer[i] = nums[j];
        //     }
        // }
        // answer

        // 方法3
        // 使用stack来再次减少操作次数
        // 怎么想到用stack的，这个很重要
        // 例子数据台上，我们造一个数据，例如：7，1，5，9，8，1
        // [7,1,5,9,8,1]，然后我们观察一下在什么情况下我们可以得到正确答案
        //  \ /5 /9 \ \
        // 我们发现当数据在下降的时候，我们没法得到正确答案，而数据在上升的时候就能立刻得到正确答案
        // 也就是说，我们需要暂时存储一下下降的数据，当出现上升的时候，我们有了正确答案就把那个数据移除
        // 而要移除的数据正好是最后一个下降的数据
        // 结合我们方法2的重复技巧，用这个例子来看看栈的顺序,
        // [],[7,1,5,9,8,1,7,1,5,9,8,1]
        // 7 vs []  -> [7]
        // 1 vs [7] -> [1,7]
        // 5 vs [1] -> [7]
        //   vs [7] -> [5,7]
        // 9 vs [5] -> [7]
        //   vs [7] -> [9]
        // 8 vs [9] -> [9]
        // ...直到最后找不到更大的了
        let n = nums.len();
        let nums = nums.repeat(2);
        let mut stack = Vec::new();
        let mut answer = vec![-1; n];
        for i in 0..n * 2 {
            while !stack.is_empty() && nums[stack[stack.len() - 1]] < nums[i] {
                answer[stack.pop().unwrap()] = nums[i];
            }
            if i < n { stack.push(i); }
        }
        answer
    }
}