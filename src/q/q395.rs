use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn longest_substring(s: String, k: i32) -> i32 {
        // 方法1
        // 通过例子我们知道，我们需要一段子串，这个串里面的所有的字符频率都要不小于K
        // 而s中可能有1-N个这样的子串，很好，我们要分割这些子串，然后处理
        // 嗯，这不就是D&Q，而分治自然和递归是好基友，递归让我们更专注解题框架上，
        // 那么我们来关注什么是不可分割的，很显然，长度本身就比k小，自然就不能分割了
        // 这自然就是递归的出口之一
        // 接下来就是怎么处理了，现在我们得到了一段子串
        // 首先我们要知道这些子串中每个字符的频率，这样，我们才能知道子串是不是合格的
        // 然后迭代子串，当出现的字符的频率不满足条件的时候，例如在第j的位置，
        // 也就是lo --- j -----hi，lo..j我们知道j不合格，
        // 但是lo..j-1我们不知道是否合格了，因为我们的范围从lo..hi缩小到了lo..j-1
        // 也就是说本来lo..j-1中可能合格的字符也不合格了，那么怎么办呢？
        // 等等，lo..j-1不就是一段子串吗？很好，我们直接将这段子串拿去递归就行了。
        // 好了，接下来，我们从j+1开始继续，重复上面的，直到迭代结束
        // 现在迭代结束了，有两个情况
        // 第一个，这段子串都是合格的，也就是lo..hi都是合格的，那么自然返回hi - lo + 1
        // 第二个，这段子串至少一段不合格，那么我们有最后一处的起点j+1，那么也就是说
        //       j+1..hi这段我们还没处理，因为迭代到hi就结束了，所以我们还要把
        //       j+1..hi这段放到递归中继续处理
        // AC 0ms 2.1mb
        fn part(s: &Vec<u8>, lo: usize, hi: usize, k: usize) -> usize {
            if hi + 1 - lo < k { return 0; }

            let mut freq = vec![0; 26];
            for i in lo..=hi { freq[s[i] as usize - 97] += 1; }

            let mut i = lo;
            let mut max = 0;
            for j in lo..=hi {
                if freq[s[j] as usize - 97] < k {
                    max = max.max(part(s, i, j - 1, k));
                    i = j + 1;
                }
            }
            if i == lo {
                hi + 1 - lo
            } else {
                max.max(part(s, i, hi, k))
            }
        }

        let n = s.len();
        part(&s.into_bytes(), 0, n - 1, k as usize) as i32
    }
}