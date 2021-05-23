use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn maximize_xor(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        // 方法1
        // 首先排序，然后二分查找每个limit，然后依次计算出最大的xor值
        // O(nlogn + n*(logn+n))
        // TLE
        // let mut nums = nums;
        // nums.sort_unstable();
        // let mut answer = vec![0; queries.len()];
        // for i in 0..queries.len() {
        //     if queries[i][1] < nums[0] {
        //         answer[i] = -1;
        //         continue;
        //     }
        //     let n = match nums.binary_search(&queries[i][1]) {
        //         Ok(n) => n + 1,
        //         Err(n) => n,
        //     };
        //     let mut sum = 0;
        //     for j in 0..n {
        //         let s = nums[j] ^ queries[i][0];
        //         sum = sum.max(s);
        //     }
        //     answer[i] = sum;
        // }
        // answer

        // 方法2
        // 优化方法1，通过字典树来过滤最佳匹配
        // 因为是异或最大，那么意味着，如果当前位是0，那么必然最佳的对位是1，
        // 字典树过滤最佳匹配的方式就是逐个位置寻找最佳对位，这样会出现两种情况：
        //    找到最佳的：继续向前，这样就淘汰掉一部分数字
        //    没有找到最佳的：继续向前，这样这轮不淘汰
        // 最后所有32位完成之后的那个数字就是最佳的数字，我们可以得到异或结果
        // 但这样会有一个问题，那就是我们可能会找到一个大于queries[i][1]的数字
        // 为了避免这个情况，我们需要做的事情就是将nums和queries都排序
        // 这样一来，我们每次可以刚刚好将<=queries[i][1] 的数字放到字典树中，就不会发生大于的情况了
        // AC 384ms 95.1mb
        struct TrieNode {
            next: Vec<Option<TrieNode>>,
        }

        let mut nums = nums;
        nums.sort_unstable();
        let mut queries = queries;
        queries.iter_mut().enumerate().for_each(|(i, v)| v.push(i as i32));
        queries.sort_unstable_by(|a, b| a[1].cmp(&b[1]));

        let mut trie = TrieNode { next: vec![None, None] };

        let mut nums_i = 0;
        for i in 0..queries.len() {
            if queries[i][1] < nums[0] {
                queries[i].push(-1);
                continue;
            }
            while nums_i < nums.len() && queries[i][1] >= nums[nums_i] {
                let mut trie_ptr = &mut trie;
                for nums_j in (0..32).rev() {
                    let bit = ((nums[nums_i] >> nums_j) & 1) as usize;
                    if trie_ptr.next[bit].is_none() {
                        trie_ptr.next[bit] = Some(TrieNode { next: vec![None, None] });
                    }
                    trie_ptr = trie_ptr.next[bit].as_mut().unwrap();
                }
                nums_i += 1;
            }

            let x = queries[i][0];
            let mut trie_ptr = &trie;
            let mut y = 0;
            for j in (0..32).rev() {
                let best = 1 - ((x >> j) & 1) as usize;
                if trie_ptr.next[best].is_none() {
                    y = y | ((1 - best) << j) as i32;
                    trie_ptr = trie_ptr.next[1 - best].as_ref().unwrap();
                } else {
                    y = y | (best << j) as i32;
                    trie_ptr = trie_ptr.next[best].as_ref().unwrap();
                }
            }
            queries[i].push(x ^ y);
        }

        println!("{:?}", queries);

        queries.sort_unstable_by(|a, b| a[2].cmp(&b[2]));
        queries.into_iter().map(|x| x[3]).collect()
    }
}