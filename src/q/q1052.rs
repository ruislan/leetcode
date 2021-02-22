use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn max_satisfied(customers: Vec<i32>, grumpy: Vec<i32>, x: i32) -> i32 {
        // 方法1
        // 窗口大小已经被固定成x了，那么剩下的就是
        // 转换成在x长度时间内因为老板生气而不满意的顾客最多的数量
        // 再加上满意的顾客的数量就是结果
        // AC 8ms 2.2mb
        let k = x as usize;
        let n = customers.len();
        let mut lo = 0;
        let mut answer = 0;
        for hi in 0..k {
            if grumpy[hi] == 1 {
                answer += customers[hi];
            }
        }
        let mut window_max = answer;
        for hi in k..n {
            window_max += if grumpy[hi] == 1 { customers[hi] } else { 0 };
            window_max -= if grumpy[lo] == 1 { customers[lo] } else { 0 };
            answer = answer.max(window_max);
        }
        grumpy.iter().enumerate().filter(|&(i, &x)| x == 0).map(|(i, _)| customers[i]).sum::<i32>() + answer
    }
}