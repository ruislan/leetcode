use crate::q::Solution;

#[allow(unused)]
impl Solution {
    // 方法有重名，改为decode_ii
    pub fn decode_ii(encoded: Vec<i32>) -> Vec<i32> {
        // 方法1
        // 回忆一下xor的规则之一，x xor y = z, x xor z = y，y xor z = x，
        // 例如: 10^01=11, 11^10=01, 11^01=10
        // 这题比那个简单题难一点就是没有给你第一个数字
        // 也就是说我们必须猜测第一个数字是多少
        // 首先想到的肯定是我们从1开始到n，直到符合的encode
        // 这里还要注意一个情况，题目要求是前n个正整数的排列，也就是说每个数字只能出现一次
        // TLE O(n^2)
        // let n = encoded.len() as i32 + 1;
        // for first in 1..n {
        //     let mut perm = vec![first];
        //     let mut seen = vec![false; n as usize];
        //     seen[first as usize - 1] = true;
        //     let mut found = true;
        //     for i in 0..encoded.len() {
        //         let x = perm[perm.len() - 1] ^ encoded[i];
        //         if x <= 0 || x > n {
        //             found = false;
        //             break;
        //         }
        //         if seen[x as usize - 1] {
        //             found = false;
        //             break;
        //         }
        //         seen[x as usize - 1] = true;
        //         perm.push(x);
        //     }
        //     if found {
        //         return perm;
        //     }
        // }
        // vec![]

        // 方法2
        // 由于数据量是10^5，所以TLE是必然的，那么我们需要优化一下
        // 我能想到的第一个就是二分搜索，从1..n之间选择mid，但是这很明显我们无法决定是往高区还是低区
        // 那么这只能回到题目本身去找线索，这个时候发现了一个重要条件n是奇数（人家还给了加粗，居然没注意）
        // 那么原数组为啥要是奇数呢，encode是个偶数，
        // 这两个是不是找出first数字的关键，如果是的话怎么来处理？
        // 我们来看看例子：
        // encoded =   [6,5,4,6]
        // perm    = [2,4,1,5,3]
        // 这里，我们一对齐，发现要是有办法把[6,5,4,6]和[4,1,5,3]给抵消掉是不是就知道2了
        // 那么[6,5,4,6] = [2^4,4^1,5^4,3^5]，这里我们发现4^1和3^5就正好包含了[4,1,5,3]
        // 那么，也就意味着我们用[2^4^1^5^3] ^ [4^1^5^3]
        // 这样利用x ^ x = 0, x ^ 0 = x的特点，我们就能留下2了。
        // 也就是说我们perm全部异或，然后encoded异或奇数就可以了，然后这两个结果再异或就得到了first,
        // 那么知道了这个规律，我们就把复杂度降低到O(n)了。
        // AC 60ms 3.6mb
        let perm_xor = (1..=encoded.len() + 1).fold(0_i32, |acc, x| acc ^ x as i32);
        let encoded_xor = (0..encoded.len()).fold(0_i32, |acc, i|
            acc ^ if i & 1 == 1 { encoded[i] } else { 0 },
        );
        let mut perm = vec![perm_xor ^ encoded_xor];
        for i in 0..encoded.len() {
            perm.push(perm[i] ^ encoded[i]);
        }
        perm
    }
}