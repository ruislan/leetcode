use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        // 方法1
        // hashset
        // AC 0ms 2.2mb 32/32
        // let mut set = std::collections::HashSet::new();
        // for x in nums {
        //     if !set.insert(x) { set.remove(&x); }
        // }
        // set.drain().collect()

        // 方法2
        // 空间为常数级，那么需要位运算
        // 我们从简单的开始来，如果只有1个数字是不同的，其他的都是2个数字相同，那么通过xor的特点，我们全部来一次，就得到了结果
        // 现在，有两个数字不同的，那么全数组异或之后得到了两个不同数字异或的结果，例如1,2,1,3,2,5得到的是3 xor 5的结果
        // 现在我们要继续走下去的话，就需要一种方法，把这两个数字分开就好了，或者有办法把这个数组分成两个简单的情况就好了
        // 这个时候我们知道3 xor 5这个数，至少有一个位置上的两个是互斥的，也就是0011 xor 0101 = 0110，
        // 这个数字0[11]0必然在括号括起来的位置是互斥的，那么我们就可以用这两个位置的任一一位是1的数字去分组两个简单的数组
        // 这里我们有一个简单的方式就是 x & -x 的方式或者 x & (!x + 1)的方式就可以得到最右边的那个1
        // 我们得到的这个数字假设为mark，那么再用这个mark去刷一次数组，
        // 由于其他的数字是两个数字，异或之后都为0，所以可以不用考虑他们，那么唯一要考虑的就是这两个不同的数字a和b
        // 而其中必然有一个数字在这个mark位置上是一样的，由于这个mark只有这个位置上是1，不一样的是0
        // 那么是0的那个必然 and mask = 0，因为mask只有这个位置是1，而那个数在这个位置上是0，那么必然结果只能是0
        // 这样一来，我们就把这两个数字区分开了
        // AC 0ms 2.2mb 32/32
        let y = nums.iter().fold(0, |acc, &x| acc ^ x);
        let mask = y & -y;
        vec![
            nums.iter().filter(|&&x| x & mask == 0).fold(0, |acc, &x| acc ^ x),
            nums.iter().filter(|&&x| x & mask != 0).fold(0, |acc, &x| acc ^ x),
        ]
    }
}