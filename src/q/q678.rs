use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn check_valid_string(s: String) -> bool {
        // 方法1
        // 括号匹配这样的情况我们可以设置一个balance，出现"("，+1，出现"）"，-1，最后看是否平衡
        // 而现在的变数是出现了"*"，它可以+1，也可以-1，也可以不加不减
        // 所以我们调整一下思路，我们需要多一个数据来记录平衡值，我们记为min_l,max_l
        // 也就是允许这个平衡值在一定范围内
        // 如果出现"（"，两个值同时+，如果出现"）"，则两个值同时-，
        // 如果出现"*"，则1个+1，1个-1
        // 当我们的max_l都小于0的时候，肯定不正确
        // 当我们的min_l可能小于0的时候，我们要保证其为0
        // 最后，检查min_l是否为0，也就是是否有平衡的可能性
        // AC 0ms 2mb 83/83
        let mut min_l = 0;
        let mut max_l = 0;
        for char in s.chars() {
            if char == '(' {
                min_l += 1;
                max_l += 1;
            } else if char == ')' {
                min_l = (min_l - 1).max(0);
                max_l -= 1;
                if max_l < 0 { return false; }
            } else {
                min_l = (min_l - 1).max(0);
                max_l += 1;
            }
        }
        min_l == 0
    }
}
