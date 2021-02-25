use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        // 方法1
        // 这题本身应该用回溯算法，但是因为每个数字只能用1次
        // 所以我们可以采用传统组合方式，即在满足条件的情况下，
        // 当前数字与前面所有的组合再进行组合，最后留下合适的组合即可
        // 当然删除重复组合这些都是常规操作了
        // 如果用回溯算法需要用到hashmap来记录当前还可以使用的数字，
        // 递归的处理都是一样的，出口就是target相等或者大于了target，如果小于target就继续递归
        // let mut arr: Vec<(i32, Vec<i32>)> = Vec::new();
        // let mut answer = Vec::new();
        // for candidate in candidates {
        //     if candidate > target { continue; }
        //     let mut new_arr = vec![(candidate, vec![candidate])];
        //     for (sum, v) in arr.iter() {
        //         let new_sum = *sum + candidate;
        //         if new_sum > target { continue; }
        //         let mut v = v.clone();
        //         v.push(candidate);
        //         if new_sum == target {
        //             v.sort_unstable();
        //             answer.push(v);
        //         } else if new_sum < target {
        //             new_arr.push((new_sum, v));
        //         }
        //     }
        //     if candidate == target { answer.push(vec![candidate]); }
        //     new_arr.into_iter().for_each(|pair| arr.push(pair));
        // }
        // answer.sort_unstable();
        // answer.dedup();
        // answer

        // 方法2
        // 还是用回溯重新写了一次
        fn backtrace(path:&mut Vec<i32>, candidates:&Vec<i32>, idx:usize, sum:i32, target: i32, answer:&mut std::collections::HashSet<Vec<i32>>) {
            if sum == target {
                answer.insert(path.clone());
                return;
            }
            for i in idx..candidates.len() {
                if sum + candidates[i] <= target {
                    path.push(candidates[i]);
                    backtrace(path, candidates, i + 1, sum + candidates[i], target, answer);
                    path.pop();
                }
            }
        }
        let mut answer = std::collections::HashSet::new();
        let mut candidates = candidates;
        candidates.sort_unstable();
        backtrace(&mut Vec::new(), &candidates, 0, 0, target, &mut answer);
        answer.into_iter().collect()
    }
}