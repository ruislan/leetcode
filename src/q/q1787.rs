use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn min_changes(nums: Vec<i32>, k: i32) -> i32 {
        // 方法1
        // 这题真的是太难了，几个小时后，我放弃了，为了打卡，直接抄大神作业吧
        // 后面再来细细的品，品出味道了，再来做一次
        let n = nums.len();
        let k = k as usize;
        let max = 1024;
        let mut f = vec![vec![0x3f3f3f3f; max]; k];
        let mut g = vec![0x3f3f3f3f; k];
        for i in 0..k {
            let mut cnt = 0;
            let mut freq = std::collections::HashMap::new();
            for j in (i..n).step_by(k) {
                *freq.entry(nums[j] as usize).or_insert(0) += 1;
                cnt += 1;
            }
            if i == 0 {
                for xor in 0..max {
                    f[0][xor] = f[0][xor].min(cnt - *freq.get(&xor).unwrap_or(&0));
                    g[0] = g[0].min(f[0][xor]);
                }
            } else {
                for xor in 0..max {
                    f[i][xor] = g[i - 1] + cnt;
                    for &cur in freq.keys() {
                        f[i][xor] = f[i][xor].min(f[i - 1][xor ^ cur] + cnt - *freq.get(&cur).unwrap_or(&0));
                    }
                    g[i] = g[i].min(f[i][xor]);
                }
            }
        }
        f[k - 1][0]
    }
}