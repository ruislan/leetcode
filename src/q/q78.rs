use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        // 方法1
        // 回溯法，套模板就行了
        // 这里由于每个情况都是结果，所以每个情况都要保存一下
        // 然后就是退出条件就是选择列表没有可以再有可选的即可
        // 由于输入的数据互不相同，所以我们只需要先排序即可。
        fn backtrace(arr:&mut Vec<i32>, nums: &Vec<i32>, i:usize, answer:&mut Vec<Vec<i32>>) {
            answer.push(arr.clone());
            for j in i..nums.len() {
                arr.push(nums[j]);
                backtrace(arr, nums, j + 1, answer);
                arr.pop();
            }
        }
        let mut answer = Vec::new();
        let mut nums = nums;
        nums.sort_unstable();
        backtrace(&mut Vec::new(), &nums, 0, &mut answer);
        answer
    }
}