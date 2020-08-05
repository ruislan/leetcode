use crate::offer::Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        // 方法1
        // 其他数字都出现了3次，一个数字只出现了1次
        // 本题没有空间限制，所以我们可以使用hashmap保存数字
        // 然后找到那个只出现了一次的数字
        // Passed 4ms 2.3mb
        // let mut map = std::collections::HashMap::new();
        // nums.iter().for_each(|&i| {
        //     let count = map.entry(i).or_insert(0);
        //     *count += 1;
        // });
        // map.into_iter().find(|&(k, v)| v == 1).map_or(0, |(k, v)| k)

        // 方法2
        // rust中，排序是很快的，反而hashmap计算量比排序还慢
        // 所以这里我们可以对数组进行排序
        // 然后线性查找那个只出现了1次的数字
        // Passed 4ms 2.2mb
        // let mut nums = nums;
        // nums.sort_unstable();
        // let mut cur = nums[0];
        // let mut count = 1;
        // for i in 1..nums.len() {
        //     if nums[i] == cur { count += 1; } else {
        //         if count == 1 { break; }
        //         cur = nums[i];
        //         count = 1;
        //     }
        // }
        // cur

        // 方法3
        // 如果除了那个只出现一次的数，其他出现三次，那么说明其他三个的和都能被3整除
        // 这类型的题目考察的是位运算，我们将所有数字的每一位的和来除以3的余数，
        // 最后留下的二进制就是我们要找的那个数
        // 这个思想我叫它放大思想，如果在这个粒度上解决不了问题，就把这个粒度放大到更细的粒度去思考
        // 相反，也有缩小粒度到更粗的粒度去思考
        // Passed 4ms 2.1mb
        // let mut bits = [0; 32];
        // nums.iter().for_each(|n| {
        //     let mut n = *n;
        //     (0..32).for_each(|j| {
        //         bits[j] += n & 1;
        //         n >>= 1;
        //     });
        // });
        // let mut res = 0;
        // bits.iter().rev().for_each(|&x| {
        //     res <<= 1;
        //     res |= x % 3;
        // });
        // res

        // 方法4
        // 根据方法3，我们知道了实际上就是对每位二进制的和取3的余数，最后留下的二进制即是那个单独的数字
        // 那么每位二进制的变化实际上就只有0,1,2三种，因为到3就会被整除，余数就是0
        // 而又由于最后只有单独一个数字，那么意味着实际上二进制的每一位的最终状态实际上都只会在0,1之间，2这个状态只会是一个中间态
        // 所以最后留下的每一位的状态组成的二进制即是结果
        // 现在我们来聚焦“一位”二进制，因为状态是0,1,2三种，实际上二进制只能记录0,1，所以我们需要一个辅助的数字来记录状态2
        // 所以0,1,2三种状态就变成了00,01,10 这三种状态，他们的转化原理是
        // 如果输入的是0，那么00,01,10，不变
        // 如果输入的是1，那么00->01,01->10,10->00，（10不能变为11，因为没有11的状态）
        // 我们用 two 和 one 来表达状态的两位数字，假设当前状态是two, one，我们计算one的方法就是：
        // 伪代码就是：
        // one =
        //  if two = 0: (00, 01)
        //      if x = 0: one = one  ((00, 01) => (00, 01))
        //      if x = 1: one = !one ((00, 01) => (01, 10))
        //  if two = 1: (10)
        //      two = 0  ((10) => (10), (10) => (00))
        // 我们再利用位运算的特性 n ^ 0 = n, n ^ 1 = !n来简化上面的代码得到
        // one =
        // if two = 0:
        //    one = one ^ x
        // if two = 1:
        //    one = 0
        // 再利用位运算的特性进行简化n & 0 = 0, n & 1 = n得到：
        // one = one ^ x & !two;
        // 得到one的状态转移公式之后，我们再计算two的，实际上two和one正好相反，得到
        // two = two ^ x & !one;
        // 通过计算单个二进制的状态变化，我们能够通过N次转化变成最终态，
        // 因为每次操作都是32个位同时进行状态变化，所以我们其实可以利用两个32位的整数来记录整体状态
        // ones:i32来记录状态第一位，twos:i32来记录状态第二位（想象一下把i32当数组用）
        // 这样我们只需要整体进行状态变化公式套用即可，
        // 然后又由于之前说到，因为只有一个数只有一次，所以他的每个二进制不是0即是1，状态只可能是00和01，
        // 那也就是说twos中的每位最终态都是0，所以我们只需要返回ones即可。
        // 0ms 2.3mb
        let (mut ones, mut twos) = (0, 0);
        for num in nums {
            ones = ones ^ num & !twos;
            twos = twos ^ num & !ones;
        }
        return ones;
    }
}

#[test]
fn test() {
    assert_eq!(Solution::single_number(vec![]), 0);
    assert_eq!(Solution::single_number(vec![3, 4, 3, 3]), 4);
    assert_eq!(Solution::single_number(vec![9, 1, 7, 9, 7, 9, 7]), 1);
    assert_eq!(Solution::single_number(vec![5, 2, 2, 2, 5, 5, 4]), 4);
    let mut v = (100000..103000).collect::<Vec<i32>>().repeat(3);
    v.push(9090);
    assert_eq!(Solution::single_number(v), 9090);
}