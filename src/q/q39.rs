use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        // 方法1
        // 组合、排列等等问题都是回溯算法的范畴
        // 回溯其实是更优雅的暴力罢了
        // 递归常伴回溯
        // 这里递归的出口就在于只要数字的和大于target，我们就不用再处理了
        // 那如果和正好等于target，我们将结果加入到结果集中，这条路后续也不用再处理了
        // 那么由于允许数字重复，所以递归的处理上，每次都要将所有的候选数字都遍历
        // 这里可以将候选数字排序，这样一来，如果我们遇到第i个数字的和大于target的，
        // i后面的所有数字都可以取消，因为必然比这个和更大
        // 最后由于不能出现重复的组合，我们可以将每个结果排序，然后删除重复项，
        // Rust有dedup()方法，kotlin有distinct()方法，实在不行就用hashset过滤一次
        
        fn combine(sum: i32, arr: Vec<i32>, candidates: &Vec<i32>, target: i32, answer: &mut Vec<Vec<i32>>) {
            for &x in candidates {
                let new_sum = sum + x;
                if new_sum > target { break; }

                let mut arr = arr.clone();
                arr.push(x);
                if new_sum == target {
                    answer.push(arr);
                } else {
                    combine(new_sum, arr, candidates, target, answer);
                }
            }
        }

        let mut answer = Vec::new();
        let mut candidates = candidates;
        candidates.sort_unstable();
        combine(0, Vec::new(), &candidates, target, &mut answer);
        for v in answer.iter_mut() {
            v.sort_unstable();
        }
        answer.dedup();
        answer
    }
}