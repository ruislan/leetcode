use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn maximum_element_after_decrementing_and_rearranging(arr: Vec<i32>) -> i32 {
        // 方法1
        // 首先排序数组，这样我们可以保证后面的数字减去前面的数字一定为正数
        // 由于我们要找最大的那个元素，所以我们维护一个水平面，
        // 这个水平面最好是正好高于前面那个数字，即是arr[i - 1] + 1
        // 但是这里有个前提就是arr[i] - arr[i - 1] > 1才可以
        // 最后要注意的地方就是第一个元素一定要为1，所以不为1的直接降级到1即可
        // O(n)
        // AC 8ms 3.3mb
        let mut arr = arr;
        arr.sort_unstable();
        let n = arr.len();
        if arr[0] != 1 { arr[0] = 1; }
        for i in 1..n {
            if arr[i] - arr[i - 1] > 1 {
                arr[i] = arr[i - 1] + 1;
            }
        }
        arr[n - 1]
    }
}