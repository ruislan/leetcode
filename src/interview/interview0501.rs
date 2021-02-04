use crate::interview::Solution;

#[allow(unused)]
impl Solution {
    pub fn insert_bits(n: i32, m: i32, i: i32, j: i32) -> i32 {
        // 方法1
        // 例如：n = 0b10001111101， i = 1, j = 5, m = 0b101
        // 首先将i右边的数据保留，利用mask = 0b1..1（一共i个)，
        //    将n & mask 得到 right = 0b1
        // 然后将n右移i得到n = 0b1000111110，
        // 然后我们将i~j这一段全部变成0，利用mask = ~0b1..1(j - i + 1个1再取反），
        //    将n & mask得到，n = 0b1000100000
        // 然后将n与m做与操作得到，n | m = 0b1000100101
        // 然后将n左移i，得到n << i = 0b10001001010
        // 最后将right与n做与操作得到, n | right = 0b10001001011
        // 注意的是，将n变成u32，这样可以不用考虑带符号进行移位
        // AC 0ms 2mb
        let (i, j) = (i as u32, j as u32);
        let mut n = n as u32;
        let mut m = m as u32;
        let get_mask = |i: u32| -> u32 {
            let mut mask = 0;
            for _ in 0..i {
                mask <<= 1;
                mask += 1;
            }
            mask
        };
        let right = n & get_mask(i);
        n = n >> i;
        n = n & !get_mask(j - i + 1);
        n = n | m;
        n = n << i;
        n = n | right;
        n as i32
    }
}