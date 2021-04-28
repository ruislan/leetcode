use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn remove_kdigits(num: String, k: i32) -> String {
        // 方法1
        // 单调栈
        // 我们想要删除最大的那k个，这k个数字尽量是高位的
        // 所以我们遍历num，当出现num[i] >= 前一个数字的时候
        // 我们就要后面的位数是否和k相等或大于，如果大于，说明我们不一定会删掉这个数字
        // 如果小于，那么我们一定删除这个数字
        // 删除的过程就是一直删除到小于等于栈内的数字或者为空或者k==0为止
        // AC 0ms 2mb
        // let n = num.len();
        // let mut k = k as usize;
        // if n == k {
        //     return String::from("0");
        // }
        // let mut nums: Vec<i32> = num.bytes().map(|x| x as i32 - b'0' as i32).collect();
        // let mut stack = Vec::new();
        // for i in 0..n {
        //     if 0 == k || stack.is_empty() {
        //         stack.push(nums[i]);
        //         continue;
        //     }
        //     if nums[i] >= stack[stack.len() - 1] {
        //         if n - 1 - i >= k {
        //             stack.push(nums[i]);
        //         } else {
        //             k -= 1;
        //         }
        //         continue;
        //     }
        //     while k > 0 && !stack.is_empty() && nums[i] < stack[stack.len() - 1] {
        //         stack.pop();
        //         k -= 1;
        //     } 
        //     stack.push(nums[i]);
        // }
        // let mut answer = String::new();
        // for x in stack {
        //     if answer.is_empty() && x == 0 { continue; }
        //     answer.push((x as u8 + b'0') as char);
        // }

        // if answer.is_empty() { return String::from("0"); } else { answer }

        // 方法2
        // 方法1写得挺繁琐，重构一下
        // AC 0ms 2mb
        // P.S 重构后居然和官方答案一样了，囧
        let n = num.len();
        let mut k = k as usize;
        if n == k { return String::from("0"); }
        let mut nums: Vec<i32> = num.bytes().map(|x| x as i32 - b'0' as i32).collect();
        let mut stack = Vec::new();
        for i in 0..n {
            while k > 0 && !stack.is_empty() && nums[i] < stack[stack.len() - 1] {
                stack.pop();
                k -= 1;
            }
            stack.push(nums[i]);
        }
        for i in 0..k { stack.pop(); }
        let mut answer = String::new();
        for x in stack {
            if answer.is_empty() && x == 0 { continue; }
            answer.push((x as u8 + b'0') as char);
        }
        if answer.is_empty() { return String::from("0"); } else { answer }
    }
}
