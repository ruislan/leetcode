use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn count_bits(num: i32) -> Vec<i32> {
        // 方法1
        // 利用库函数，这个题应该是不能用库函数的
        // AC 8ms 2.4mb
        // let mut answer = Vec::new();
        // for x in 0..=num {
        //     answer.push(x.count_ones() as i32);
        // }
        // answer

        // 方法2
        // 利用移位操作来计算1的数量，
        // O(sizeof(x) * n)
        // AC 0ms 2.4mb
        // let mut answer = Vec::new();
        // for mut x in 0..=num {
        //     let mut ones = 0;
        //     while x != 0 {
        //         if x & 1 == 1 { ones += 1; }
        //         x >>= 1;
        //     }
        //     answer.push(ones);
        // }
        // answer

        // 方法3
        // 我们来观察一下0-7这几个数字
        // 0: 0       0
        // 1: 1       1
        // 2: 10      1
        // 3: 11      2
        // 4: 100     1
        // 5: 101     2
        // 6: 110     2
        // 7: 111     3
        // 这里我们看到当数字能够被2整除的时候，1的个数正好是和整除后的那个数字相等的
        // 例如: 2/2 =1 ,所以1和2都只有1个1，6/2=3，所以6和3都有2个1.
        // 那么也就意味着偶数我们可以这样处理了，那么奇数呢
        // 3/2=1，1有1个1，而3有2个1，仿佛是比被整除的那个1的个数要多1，再试试
        // 5/2=2,2有1个1，而5有2个1，也对
        // 7/2=3,3有2个1，而7有3个1，也正好是2+1=3
        // 那么规律已经出来了，如果我们当前处理的数字设为x，设y = x/2
        // 那么y就是被除以后的数字，
        // 如果能够被2整除，那么ones(x) = ones(y)
        // 如果不能被整除，那么ones(x) = ones(y) + 1
        // AC 8ms 2.4mb
        // P.S 其实在数据量较大时，方法2会稍快，因为不需要分配空间
        let mut num = num as usize;
        let mut answer = vec![0; num + 1];
        for i in 1..=num {
            // if i & 1 == 1 {
            //     answer[i] = answer[i >> 1] + 1;
            // } else {
            //     answer[i] = answer[i >> 1];
            // }
            // 提交是用注释那一块的，下面的缩减了以下
            answer[i] = answer[i >> 1] + (i as i32 & 1)
        }
        answer
    }
}