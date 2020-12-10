use crate::interview::Solution;

#[allow(unused)]
impl Solution {
    pub fn reverse_bits(num: i32) -> i32 {
        // 方法1
        // 逐个数据计算查看是1还是0
        // 如果是1，累加1，如果是0：
        //   将前一个0的左边的1的统计和现在0的左边的1的统计（也就是前一个0的右边的1的统计）相加再加上1
        //   与最大比较，留下最大的那个，数字没有了
        // 最后可能由于有都是1或者0的情况，我们还需要再统计一次并比较
        // 还有一种全是1的情况，这会让统计大于32位，我们取32位就行了
        // 值得注意的是由于RUST没有">>>"操作，所以先将i32转换成u32，这样就可以操作了
        // Passed 0ms 1.9mb
        // let mut num = num as u32;
        // let (mut last_ones, mut ones) = (0, 0);
        // let mut answer = 0;
        // while num > 0 {
        //     if num & 1 == 1 {
        //         ones += 1;
        //     } else {
        //         answer = answer.max(last_ones + ones + 1);
        //         last_ones = ones;
        //         ones = 0;
        //     }
        //     num = num >> 1;
        // }
        // answer = answer.max(last_ones + ones + 1);
        // answer = answer.min(32);
        // answer

        // 方法2
        // 方法1的优化版本，思路是一样的
        // 变化的地方是
        // 第一个直接32位，每次多移一位到当前位处理，这样就可以避免还有前导0的情况，省略了26和28行代码
        // 第二个是比较放在每次迭代的时候，好处是无论来的是1还是0，我们都可以抓住最大的那个
        // 如果是1，我们直接加1
        // 如果是0，我们就将前面的1统计设置为当前的1统计和0自身，然后清空当前的1统计
        // 然后每次迭代都比较前面的1统计（带0）和当前的1统计与当前最大值，留下最大的那个
        // Passed 0ms 2mb
        let mut num = num as u32;
        let (mut last_ones, mut ones) = (0, 0);
        let mut answer = 0;
        for i in 0..32 {
            if (num >> i) & 1 == 1 {
                ones += 1;
            } else {
                last_ones = ones + 1;
                ones = 0;
            }
            answer = answer.max(last_ones + ones);
        }
        answer
    }
}

#[test]
fn test() {
    assert_eq!(Solution::reverse_bits(0), 1);
    assert_eq!(Solution::reverse_bits(1), 2);
    assert_eq!(Solution::reverse_bits(2), 2);
    assert_eq!(Solution::reverse_bits(3), 3);
    assert_eq!(Solution::reverse_bits(4), 2);
    assert_eq!(Solution::reverse_bits(-1), 32);
    assert_eq!(Solution::reverse_bits(-2), 32);
    assert_eq!(Solution::reverse_bits(-3), 32);
}