use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn largest_number(nums: Vec<i32>) -> String {
        // 方法1
        // 先看头数字，放到0-9里面去
        // 然后依次排序0-9中的数组，
        // 拼接比较的两个a,b成字符串，做字符串排序
        // 依次取出数据组成字符串
        // 注意有[0, 0]的情况
        // AC 0ms 1.9mb
        // let mut nums = nums;
        // let mut bucket = vec![vec![]; 10];
        // for &x in nums.iter() {
        //     let first = x / 10_i32.pow(x.to_string().len() as u32 - 1);
        //     bucket[first as usize].push(x);
        // }

        // for v in bucket.iter_mut() {
        //     v.sort_unstable_by(|a, b| {
        //         let mut cp_a = a.to_string();
        //         cp_a.push_str(&b.to_string());
        //         let mut cp_b = b.to_string();
        //         cp_b.push_str(&a.to_string());
        //         cp_b.cmp(&cp_a)
        //     });
        // }
        // let mut answer = Vec::new();
        // for i in (1..10).rev() {
        //     for &x in bucket[i].iter() {
        //         answer.push(x);
        //     }
        // }
        // if !answer.is_empty() {
        //     for &x in bucket[0].iter() {
        //         answer.push(x);
        //     }
        // }
        // if answer.is_empty() {
        //     return "0".to_string();
        // }
        // answer.into_iter().map(|x| x.to_string()).collect()

        // 方法2
        // 通过方法1，我们知道其实核心就是字符串拼接之后排序
        // 所以我们不需要桶排一次了，直接每个数字转换成字符串排序
        // 只需要注意全是0的数组即可
        // AC 0ms 2mb
        let mut nums = nums;
        let mut nums:Vec<String> = nums.into_iter().map(|x| x.to_string()).collect();
        nums.sort_unstable_by(|a, b| {
            let cp_a = a.clone() + &b.clone();
            let cp_b = b.clone() + &a.clone();
            cp_b.cmp(&cp_a)
        });
        let answer = nums.concat();
        if answer.starts_with("0") { 
            "0".to_string()
        } else {
            answer
        }
    }
}