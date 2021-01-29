use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        // 方法1
        // 回溯是解决组合的有力武器之一
        // 回溯一般和递归是好兄弟
        // 递归的出口就是组合的长度达到了k的长度
        // 每次递归就是将当前数字和它后面到n的所有数字进行组合（调用递归）
        // 这里为什么是当前数字和它后面到n的数字，因为1到当前数字已经在组合中了
        fn gen(i: i32, n: i32, k: usize, prev: Vec<i32>, arr: &mut Vec<Vec<i32>>) {
            if prev.len() == k { arr.push(prev); } else {
                for j in i..=n {
                    let mut cur = prev.clone();
                    cur.push(j);
                    gen(j + 1, n, k, cur, arr);
                }
            }
        }
        let mut answer = Vec::new();
        gen(1, n, k as usize, Vec::new(), &mut answer);
        answer
    }
}