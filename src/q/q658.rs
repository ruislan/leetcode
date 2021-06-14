use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        // 方法1
        // 二分查找到这个数字x，
        // 1. 如果找到，那么这个数字一定是最接近x的
        // 2. 如果没找到，我们的库函数能够得到一个刚好大于这个数的位置，
        // 然后我们用两个位置lo和hi，lo指向找到的那个数字，如果没有找到，那就是lo - 1，hi=lo+1
        // 然后中间向两边发散，如果左边没有了，就取右边；如果右边越界了就取左边；如果都还有就比较大小；
        // AC 8ms 2.1mb
        let (i, found) = match arr.binary_search(&x) {
            Ok(i) => (i, true),
            Err(i) => (i, false),
        };
        let n = arr.len();
        let mut lo = if found { i } else { i - 1 } as i32;
        let mut hi = lo + 1;
        let mut answer = Vec::new();
        for _ in 0..k {
            if lo < 0 {
                hi += 1;
            } else if hi as usize >= n {
                lo -= 1;
            } else if x - arr[lo as usize] > arr[hi as usize] - x {
                hi += 1;
            } else {
                lo -= 1;
            }
        }
        for i in lo + 1..hi {
            let i = i as usize;
            answer.push(arr[i]);
        }
        answer
    }
}
