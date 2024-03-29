use crate::q::Solution;

#[allow(unused)]
impl Solution {
    // 由于方法重名，修改为remove_duplicates_ii
    pub fn remove_duplicates_ii(nums: &mut Vec<i32>) -> i32 {
        // 方法1
        // 首先这个数组是增序排列的，也即是说出现过的数字后面不可能又出现
        // 只能用O(1)的空间，也即是说只有只能定义几个变量
        // 这里我们定义两个，一个记录当前的数字，一个记录当前数字的频率
        // 首先，如果数组的长度少于3，那么说明整个
        // 然后，我们找到首次出现超过2个的数字，作为当前替换数字的起点lo，
        //      如果我们找完了都没有超过的数字，那么说明整个数组都合格。
        // 接着，我们继续向右搜索，直到出现一个新的数字，这个数字当前频率是1
        //      我们将这个数字替换掉lo位置的数字，同时lo的位置向前移动1
        //      一直到hi到达最后。
        // 最后，lo正好就是我们合格数字的长度
        // AC 0ms 2.1mb
        // let n = nums.len();
        // let limit = 2;
        // if n <= limit { return n as i32; }
        // let mut lo = 0;
        // let mut cur_freq = 1;
        // let mut cur = nums[0];
        // for hi in 1..n {
        //     if cur == nums[hi] {
        //         cur_freq += 1;
        //     } else {
        //         cur = nums[hi];
        //         cur_freq = 1;
        //     }
        //     if cur_freq > limit {
        //         lo = hi;
        //         break;
        //     }
        // }

        // if lo < 2 { return n as i32; }

        // for hi in lo + 1..n {
        //     if cur == nums[hi] {
        //         cur_freq += 1;
        //     } else {
        //         cur = nums[hi];
        //         cur_freq = 1;
        //     }
        //     if cur_freq <= limit {
        //         nums[lo] = nums[hi];
        //         lo += 1;
        //     }
        // }
        // lo as i32

        // 方法2
        // 方法1复杂了点，我们简化一下
        // 其实我们的双指针可以从索引2开始（注意这里lo指要被替换的位置，hi指向正在检查的位置）
        // 这样我们只需要检查hi指针所指向的数字与lo指针的前2位数字（即nums[lo - 2]）是否相等
        // 如果相等，则说明是要被删除的数字
        // 如果不相等，则说明是要将nums[hi]移动到nums[lo]，同时lo的位置要向前移动一格
        // AC 0ms 2mb
        let n = nums.len();
        if n <= 2 {
            return n as i32;
        }
        let mut lo = 2;
        let mut hi = 2;
        while hi < n {
            if nums[hi] != nums[lo - 2] {
                nums[lo] = nums[hi];
                lo += 1;
            }
            hi += 1;
        }
        lo as i32
    }
}
