use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn find_best_value(arr: Vec<i32>, target: i32) -> i32 {
        // 方法1
        // 找到一个数字top，使得arr高于这个数字top都变成top，然后和与target最接近
        // 那么暴力的解法肯定是从arr中最高的那个数字到数字0逐个尝试，这是O(n^2)
        // 暴力必然TLE，那么就需要优化了，很明显，猜的数字是高到低或者低到高，有序，猜数字
        // 必然二分了，那么二分的情况下就要看中间数字与target的距离来看取低区还是高区
        // 如果中间数字使得arr的和与target相等，那么就不用再找了，这个数字必然是答案（就是target，不能更接近了)
        // 如果中间数字使得arr的和与target相等或者要大，那么说明我们数字取大了，可以取的更小（更接近target）
        // 如果中间数字使得arr的和与target相等或者要小，那么说明我们数字取小了，可以取的更大点（更接近target）
        // 那么根据上面的方法，如果没有找到最佳的情况下，
        // 我们可以找得了一个刚好比target大的数字，
        // 然后我们用这个数字-1得到一个刚好比target小的数字，
        // 然后得到哪个数字与target更接近
        // AC 0ms 2mb
        let sum = |mid: i32| -> i32 { arr.iter().map(|&x| x.min(mid)).sum() };
        let mut hi = *arr.iter().max().unwrap();
        let mut lo = 0;
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if sum(mid) == target {
                return mid;
            } else if sum(mid) > target {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }
        if (sum(lo) - target).abs() >= (sum(lo - 1) - target).abs() {
            lo - 1
        } else {
            lo
        }
    }
}