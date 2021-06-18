use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn gray_code(n: i32) -> Vec<i32> {
        // 方法1
        // 我们其实是可以逐位去将前面的所有结果加一遍0和1，例如：
        // 1位 [0｜1] ，2位[00,01｜10,11]，3位[000,001,010,011|100,101,110,111]
        // 这时候前面2位都很好，但是第三位就出问题了，011和100不对啊，我们观察一下
        // 发现前面半截都是对的，后面半截也是对的，而011和111正好也是对的，那是不是把后面半截掉头就行了啊？
        // 3位[000,001,010,011|111,110,101,100]，嗯，没问题哈
        // 4位[0000,0001,0010,0011,0111,0110,0101,0100|1100,1101,1110,1111,1011,1010,1001,1000]
        // 4位也完全正确
        // 看来这种方法可行，总结一下，就是每进一位，就将前面全部加上0，然后再全部倒过来前面加上1
        // AC 8ms 2.6mb
        // if n == 0 { return vec![]; }
        // let mut answer = vec![0, 1];
        // for i in 2..=n {
        //     let mut temp = answer.clone();
        //     temp.reverse();
        //     for x in temp {
        //         answer.push(x | (1 << (i - 1)));
        //     }
        // }
        // answer

        // 方法2
        // 大神的题解，公式法： x ^ (x >> 1)，我为什么想不到？看来还是积累不够啊。
        // AC 12ms 2.7mb
        let mut answer = Vec::new();
        for x in 0..(1 << n) {
            answer.push(x ^ (x >> 1));
        }
        answer
    }
}