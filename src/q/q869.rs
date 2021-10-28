use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn reordered_power_of2(n: i32) -> bool {
        // 方法1
        // 2的幂是有限的，1..10^9，所以我们可以存储每个2的幂
        // 因为前导数字不能为0，所以数字的位数无论怎么排序，都必须是这个位数
        // 所以我们可以依次去检查每一位置的数字使用，如果完全符合，则是匹配的
        // 总之就是一个词频的检查就可以了
        // AC 0ms 2mb 136/136
        let mut arr = Vec::new();
        let mut x = 1;
        while x <= 1000000000 {
            let mut v = vec![0; 10];
            for ch in x.to_string().chars() {
                v[(ch as u8 - '0' as u8) as usize] += 1;
            }
            arr.push(v);
            x = x << 1;
        }
        let mut v = vec![0; 10];
        for ch in n.to_string().chars() {
            v[(ch as u8 - '0' as u8) as usize] += 1;
        }
        arr.into_iter().find(|a| *a == v).is_some()
    }
}