use crate::q::Solution;

#[allow(unused)]
impl Solution {
    // 做这个题之前如果能搞定q300的话，就发现两个其实差不多
    // 我们用q300的三种方法来解决这个题
    pub fn max_envelopes(envelopes: Vec<Vec<i32>>) -> i32 {
        // 方法1
        // 回溯+剪枝，其实也是动态规划中的一种
        // AC 916ms 2.4mb
        // fn dfs(arr: &mut Vec<Vec<i32>>, envelopes: &Vec<Vec<i32>>, i: usize, memo: &mut Vec<i32>) -> i32 {
        //     if i == envelopes.len() { return 0; }
        //     if memo[i] > 0 { return memo[i]; }
        //     let mut total = 0;
        //     for j in i..envelopes.len() {
        //         if let Some(last) = arr.last() {
        //             if last[0] < envelopes[j][0] && last[1] < envelopes[j][1] {
        //                 arr.push(envelopes[j].clone());
        //             } else {
        //                 continue;
        //             }
        //         } else {
        //             arr.push(envelopes[j].clone());
        //         }
        //         total = total.max(1 + dfs(arr, envelopes, j + 1, memo));
        //         arr.pop();
        //     }
        //
        //     memo[i] = total;
        //     total
        // }
        //
        // let n = envelopes.len();
        // let mut envelopes = envelopes;
        // envelopes.sort_unstable();
        // dfs(&mut Vec::new(), &envelopes, 0, &mut vec![0; n])

        // 方法2
        // 只是这里有两个地方不一样要注意，
        // 1：这里不需要子序列，也就是说符合条件的会比q300的情况多，
        //      所以这里要先对信封从小到大的排序，避免遗漏情况
        // 2：这里的条件变成两个都要满足，而不是只有一个
        // AC 288ms 2.5mb
        // let n = envelopes.len();
        // let mut envelopes = envelopes;
        // envelopes.sort_unstable();
        // let mut dp = vec![1; n];
        // for i in 0..n {
        //     for j in 0..i {
        //         if envelopes[i][0] > envelopes[j][0] && envelopes[i][1] > envelopes[j][1] {
        //             dp[i] = dp[i].max(dp[j] + 1);
        //         }
        //     }
        // }
        // dp.into_iter().max().unwrap()

        // 方法3
        // 贪心+二分
        // 具体的解法和q300几乎一致
        // 需要注意的地方就在于，在排序的过程中，我们要将w相同的信封，按照h高到低的方式排列
        // 因为w相同的情况下，我们只能取1个，而通过h采取降序的方式，
        // 能够保证在每次迭代的时候，都只选择w相同中的一个，例如[6,7],[6,4]
        // 第一次我们选择[6,7]，而第二次我们就只选择[6,4]了
        // 如果我们按照升序排列[6,4],[6,7]
        // 第一次我们选择[6,4]的时候，[6,7]就能够进入选择，这是不合乎条件的
        // 为什么呢？因为在二维中，我们要先让一个维度彻底符合条件
        // 然后就只需要看另外一个维度，也就是说w按照升序排列之后，我们就不用看w，只看h了
        // 而只看h的情况下，就会出现上述的w相等的问题，而采用降序就忽略掉这个问题了
        // AC 4ms 2.4mb
        let n = envelopes.len();
        let mut envelopes = envelopes;
        envelopes.sort_unstable_by(|a, b| {
            let cmp = a[0].cmp(&b[0]);
            if cmp == std::cmp::Ordering::Equal {
                b[1].cmp(&a[1])
            } else {
                cmp
            }
        });
        let mut answer: Vec<i32> = Vec::new();
        for envelope in envelopes {
            if answer.is_empty() || envelope[1] > answer[answer.len() - 1] {
                answer.push(envelope[1]);
                continue;
            }
            let mut i = answer.binary_search(&envelope[1]).unwrap_or_else(|i| i);
            answer[i] = envelope[1];
        }
        answer.len() as i32
    }
}