use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        // 方法1
        // 组合呀排列呀都是回溯
        // 回溯就是套模板就行了，这一类问题都可以解决
        // 本题的出口条件是路径中的元素达到k个，而加入结果的条件是要其中的和等于n
        // 然后我们可以从1-9遍历，因为不存在重复的数字，
        // 所以，我们每次路径的选择列表都可以通过从减少已经选择过的数字来剪枝
        // 在这里就是1-3选择了，选择列表就从4-9就好了
        fn backtrace(path: &mut Vec<i32>, k: usize, sum: i32, n: i32, x: i32, answer: &mut Vec<Vec<i32>>) {
            if path.len() == k {
                if sum == n { answer.push(path.clone()); }
                return;
            }
            for i in x..=9 {
                if sum + i <= n {
                    path.push(i);
                    backtrace(path, k, sum + i, n, i + 1, answer);
                    path.pop();
                }
            }
        }
        let mut answer = Vec::new();
        backtrace(&mut Vec::new(), k as usize, 0, n, 1, &mut answer);
        answer
    }
}