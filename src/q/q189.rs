use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        // 方法1
        // 直接一个一个的移动
        // 只有一个中间变量last，满足原地移动的条件
        // 时间复杂度O(k*(n - 1))
        // Passed 232ms 2.2mb
        // let n = nums.len();
        // let k = k as usize % n;
        // for _ in 0..k {
        //     let last = nums[n - 1];
        //     for i in (1..n).rev() {
        //         nums[i] = nums[i - 1];
        //     }
        //     nums[0] = last
        // }

        // 方法2
        // 环状替换
        // 就是将nums[0] -> nums[0 + k] -> nums[0 + 2k].. -> nums[0]
        // 然后nums[1] -> nums[1 + k] .. -> nums[1]
        // 一直到所有的数字都被替换过了
        // 所以这里需要设置几个参数
        // count:已经替换的数字统计
        // ptr：当前0-k的起点
        // i: 当前要移动的数字的索引
        // cur:当前要移动的数字
        // j: 要移动到的位置的索引
        // next:要移动到的位置的数字
        // 我们通过循环来不停的移动数字，直到当前的索引i == ptr也就是回到了起点
        // Passed 0ms 2.1mb
        let n = nums.len();
        let k = k as usize % n;

        let mut ptr = 0;
        let mut count = 0;
        while count < n {
            let mut i = ptr;
            let mut cur = nums[i];
            loop {
                let j = (i + k) % n;
                let next = nums[j];

                nums[j] = cur;

                i = j;
                cur = next;

                count += 1;

                if ptr == i { break; }
            }
            ptr += 1;
        }


        // 方法3
        // 我们观察范例数组：[1，2，3，4，5，6，7] ，k = 3
        // 输出：[5,6,7,1,2,3,4]
        // 我们发现在范围[0 until k]和[k until n]两个范围
        // 如果我们分别翻转两个范围，那么就会得到：
        // [7,6,5,4,3,2,1]，正好是原数组的翻转
        // Passed 0ms 2.3mb
        // let n = nums.len();
        // let k = k as usize % n;
        //
        // nums.reverse();
        // for i in 0..k / 2 {
        //     nums.swap(i, k - 1 - i);
        // }
        // for i in k..k + (n - k) / 2 {
        //     nums.swap(i, n - 1 - i + k);
        // }

        // 方法4
        // 方法3的库函数版本
        // let n = nums.len();
        // let k = k as usize % n;
        // nums.reverse();
        // nums[0..k].reverse();
        // nums[k..n].reverse();

        // 方法5
        // 库函数直接支持旋转，:)
        // let n = nums.len();
        // let k = k as usize % n;
        // nums.rotate_right(k);
    }
}
