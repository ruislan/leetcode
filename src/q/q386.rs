use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn lexical_order(n: i32) -> Vec<i32> {
        // 方法1
        // 回溯法是解决这类需要不停尝试结果的利器
        // 这里直到1-9是第一层
        // 从第二层开始回溯，因为第二层是0-9
        // 每一次我们都将该数字提升10倍，然后加上0-9就成为了
        // 下一个需要回溯的点
        // 当然回溯一般都是用到递归的
        // 这里需要注意的是，递归的离开条件就是当前的数字
        // 大于了n，那么就不在寻找
        // AC 12ms 2.6mb
        fn gen(mut x: i32, n: i32, arr: &mut Vec<i32>) {
            if x <= n {
                arr.push(x);
                for i in 0..=9 {
                    gen(x * 10 + i, n, arr);
                }
            }
        }

        let mut answer = Vec::new();
        for i in 1..=9 {
            gen(i, n, &mut answer);
        }
        answer

        // 方法2
        // 可以将所有的数字转换成字符串
        // 然后直接排序，就是字典排序
        // 然后再转换回数字即可
        // 这个方法略微有点讨巧，应该不是本题考试目的
        // AC 20~36ms 4.7~4.9mb
        // let mut arr: Vec<String> = (1..=n).map(|x| x.to_string()).collect();
        // arr.sort_unstable();
        // arr.into_iter().map(|x| x.parse::<i32>().unwrap()).collect()
    }
}