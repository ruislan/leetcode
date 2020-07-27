use crate::q::Solution;

impl Solution {
    pub fn reverse_bits(n: u32) -> u32 {
        // 方法1
        // 利用库函数直接翻转
        // n.reverse_bits()

        // 方法2
        // 辗转相除2，余数进入栈，然后pop栈
        // 创建和num = 0
        // 记录pop的个数s，按照num += 2.pow(s) * pop()
        // 然后返回num
        // let (mut n, mut bits) = (n, Vec::new());
        // while n > 0 {
        //     bits.push(n & 1);
        //     n >>= 1;
        // }
        // while bits.len() < 32 { bits.push(0); }
        // let (mut exp, mut num) = (32, 0);
        // bits.into_iter().for_each(|base| {
        //     exp -= 1;
        //     num += base * 2_u32.pow(exp);
        // });
        // num

        // 方法3
        // 利用移位操作，每次处理最小位，注意要先移位再处理（有新数据要处理才移位），否则，最后一次处理将把第一次处理的位置移动到界外，从而导致最小位是0
        // let (mut n, mut num) = (n, 0);
        // (0..32).for_each(|_| {
        //     num <<= 1;
        //     num += n & 1;
        //     n >>= 1;
        // });
        // num

        // 方法4
        // 利用分治法，对于任意两个比特(b1,b2)，我们可以采取让每个比特&掩码，b1 & 10，b2 & 01得到(b1,0),(0,b2)
        // 然后我们移位进行位置交换得到(0,b1),(b2,0)，
        // 最后合并就成了(b2,b1)
        // 所以这个32位比特，
        // 我们可以先划分成两个16位进行交换,这里不需要掩码直接交换即可
        // 再把16位中的8位进行交换，这里需要的掩码 1111(f) 1111(f) 0000(0) 0000(0) 和 0000 0000 1111 1111
        // 再把8位中的4位进行交换， 这里需要的掩码 1111(f) 0000(0) 1111 0000 和 0000 1111 0000 1111
        // 再把4位中的2位进行交换， 这里需要的掩码 1100(c) 1100 1100 1100 和 0011(3) 0011 0011 0011
        // 再把2为中的1位进行交换， 这里需要的掩码 1010(a) 1010 1010 1010 和 0101(5) 0101 0101 0101
        // 即完成了二进制翻转
        let mut num = n >> 16 | n << 16;
        num = (num & 0xff00ff00) >> 8 | (num & 0x00ff00ff) << 8;
        num = (num & 0xf0f0f0f0) >> 4 | (num & 0x0f0f0f0f) << 4;
        num = (num & 0xcccccccc) >> 2 | (num & 0x33333333) << 2;
        (num & 0xaaaaaaaa) >> 1 | (num & 0x55555555) << 1

        // 这道题Rust没有输入，换成Kotlin或者Java来提交
        // 注意这道题如果是Java或者Kotlin是有符号的，所以这里要注意负数的情况
        // 方法1
        // 将数字转换成Long，然后处理完成之后再转换成Int即可
        // 下面是Kotlin代码
        // val bits = LongArray(32)
        // var n = if (n < 0) (n and Int.MAX_VALUE) + (2.0).pow(31).toLong() else n.toLong()
        // var i = 0
        // while (n > 0) {
        //     bits[i++] = n % 2
        //     n /= 2
        // }
        // var exp = 0
        // var num = 0L
        // for (j in 31 downTo 0) num += bits[j] * 2.0.pow(exp++).toLong()
        // return num.toInt()

        // 方法2
        // 利用左移，每次都处理最低位，然后对n使用无符号右移，因为右移1位之后，我们不再保留最高位的符号
        // 例如 100..00右移一位之后是010..00，如果保留最高位就成了110..00
        // 下面是Kotlin代码
        // var n = n
        // var num = 0
        // for (i in 0..31) {
        //     num = num shl 1
        //     num += n and 1
        //     n = n ushr 1
        // }
        // return num

        // 方法3，同rust方法3，唯一需要注意的是，右移的时候不要保留最高位的符号位，所以要使用ushr
        // var num = (n ushr 16) or (n shl 16)
        // num = ((num and 0xff00ff00.toInt()) ushr 8) or ((num and 0x00ff00ff) shl 8)
        // num = ((num and 0xf0f0f0f0.toInt()) ushr 4) or ((num and 0x0f0f0f0f) shl 4)
        // num = ((num and 0xcccccccc.toInt()) ushr 2) or ((num and 0x33333333) shl 2)
        // return ((num and 0xaaaaaaaa.toInt()) ushr 1) or ((num and 0x55555555) shl 1)
    }
}

#[test]
fn test() {
    assert_eq!(Solution::reverse_bits(0), 0);
    assert_eq!(Solution::reverse_bits(1), 2147483648);
    assert_eq!(Solution::reverse_bits(43261596), 964176192);
    assert_eq!(Solution::reverse_bits(4294967293), 3221225471);
}