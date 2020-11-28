use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn largest_perimeter(a: Vec<i32>) -> i32 {
        // 方法1
        // 排序之后，数据从小到大
        // 然后我们从后向前迭代，如果能组成三角形(a + b > c)
        // 因为从后向前，从大到小，所以只要有三角形，就是最大那个
        // 那么就返回这个周长
        // Passed 4ms 2.1mb
        let mut a = a;
        a.sort_unstable();
        for i in (0..a.len() - 2).rev() {
            if a[i] + a[i + 1] > a[i + 2] {
                return a[i] + a[i + 1] + a[i + 2];
            }
        }
        0
    }
}