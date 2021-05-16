use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn find_maximum_xor(nums: Vec<i32>) -> i32 {
        // 方法1
        // 所有两个尝试xor，找出最大的，
        // 暴力O(n^2)
        // AC 328ms 2.2mb
        // let n = nums.len();
        // let mut answer = 0;
        // for i in 0..n {
        //     for j in i + 1..n {
        //         answer = answer.max(nums[i] ^ nums[j]);
        //     }
        // }
        // answer

        // 方法2
        // 通过字典树过滤候选者，怎么想到字典树的呢？
        // 假设我们有一个 0100 的数字，我们要达到最好就是和比特位它完全相反 1011
        // 所以 1011 就是我们的期望了，那么其他数字都是我们的候选
        // 这个时候，我们逐位查看候选者是否满足这个最佳的期望，不满足的候选者实际上就被淘汰了，
        // 首先取出1，查找其他几个数字的第一位是不是1，有1的，就留下
        // 然后是0，然后是1，最后是1
        // 那假设其中所有的候选不存在最佳匹配，那表示这一次我们把所有的候选者都留下
        // 这个过程画图表示的话，就形如下：
        //          root
        //         1    0
        //       1  0  1  0
        //     1   1 0
        //          1
        // 这样看起来像是颗二叉树，但是每个叶节点的值的范围都只有[0,1]，是不是和字典树一样了
        // 可能大多数时候字典树的经典应用都是过滤单词，也就是26个字符，这里只有2个而已
        // 所以字典树就是这么联想来的，那么本题的解题思路就有了，
        // 首先构建字典树，将所有的数字按照比特位存入其中
        // 然后迭代数字i，逐位找出nums[i]的最佳，如果存在就留下，不存在就继续，直到最低位
        // 最后将所有的结果中找出最大的那个
        // 这题可以做到O(32*n)，这里我始终觉得O(32*n)与O(n)还是有点距离的，但是也勉强算O(n)
        // AC 52ms 9.9mb
        struct TrieNode {
            next: Vec<Option<TrieNode>>,
        }

        let mut trie = TrieNode { next: vec![None, None] };
        for &x in nums.iter() {
            let mut trie_ptr = &mut trie;
            for k in (0..32).rev() {
                let bit = ((x >> k) & 1) as usize;
                if trie_ptr.next[bit].is_none() {
                    trie_ptr.next[bit] = Some(TrieNode { next: vec![None, None] });
                }
                trie_ptr = trie_ptr.next[bit].as_mut().unwrap();
            }
        }
        let mut answer = 0;
        for x in nums {
            let mut candidate = 0;
            let mut trie_ptr = &trie;
            for k in (0..32).rev() {
                let best = 1 - ((x >> k) & 1) as usize;
                candidate <<= 1;
                if trie_ptr.next[best].is_some() {
                    candidate |= best as i32;
                    trie_ptr = trie_ptr.next[best].as_ref().unwrap();
                } else {
                    candidate |= 1 - best as i32;
                    trie_ptr = trie_ptr.next[1 - best].as_ref().unwrap();
                }
            }
            answer = answer.max(x ^ candidate);
        }
        answer
    }
}