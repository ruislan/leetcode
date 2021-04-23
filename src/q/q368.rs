use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn largest_divisible_subset(nums: Vec<i32>) -> Vec<i32> {
        // 方法1
        // backtrace找到所有的可能的组合
        // 这个会超时，因为后续存在大量的重复计算
        // fn dfs(path: &mut Vec<i32>, i: usize, nums: &Vec<i32>, answer: &mut Vec<i32>) {
        //     if path.len() > answer.len() {
        //         *answer = path.clone();
        //     }
        //     for j in i..nums.len() {
        //         let mut is_it = true;
        //         for &x in path.iter() {
        //             if nums[j] % x != 0 {
        //                 is_it = false;
        //                 break;
        //             }
        //         }
        //         if is_it {
        //             path.push(nums[j]);
        //             dfs(path, j + 1, nums, answer);
        //             path.pop();
        //         }
        //     }
        // }
        // let mut answer = Vec::new();
        // let mut nums = nums;
        // nums.sort_unstable();
        // dfs(&mut Vec::new(), 0, &nums, &mut answer);
        // answer

        // 方法2
        // 既然深度搜索会超时，那么我们有两个方式来解决，一个是记忆化路径并剪枝，
        // 很明显，这题记忆化路径很困难，那么就是第二个了，动态规划
        // 我们迭代每一个数字nums[i]，然后依次除以0..i位置的数据，如果能够整除，这个位置为j
        // 那么我们就将nums[j]中的存储的列表拿出来 + 自己的长度，然后和自己nums[i]存储的列表长度比较，
        // 留下最长的那个，直到内层迭代结束
        // 外层迭代直到最后一个数字结束
        // 这样我们就能从所有的存储中找到最长的那个数组
        // AC 20ms 2mb
        let mut nums = nums;
        nums.sort_unstable();
        let n = nums.len();
        let mut memo = vec![vec![]; n];
        for i in 0..n {
            memo[i].push(nums[i]);
        }
        let mut answer = 0;
        for i in 0..n {
            for j in 0..i {
                if nums[i] % nums[j] == 0 {
                    if memo[j].len() + 1 > memo[i].len() {
                        let mut v = memo[j].clone();
                        v.push(nums[i]);
                        memo[i] = v;
                    }
                }
            }
            if memo[i].len() > memo[answer].len() {
                answer = i;
            }
        }
        memo[answer].clone()
    }
}
