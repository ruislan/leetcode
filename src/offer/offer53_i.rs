use crate::offer::Solution;

#[allow(unused)]
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        // 方法1
        // 利用库函数直接统计
        // Passed 0ms 2.5mb
        // nums.iter().filter(|&&x| x == target).count() as i32

        // 方法2
        // 线性查找到该数据，然后向前统计直到没有数据
        // Passed 0ms 2.4mb
        // let mut count = 0;
        // for n in nums {
        //     if n == target { count += 1; }
        // }
        // count

        // 方法3
        // 利用有序特性，二分查找到数据，然后左右移动，直到不等于该数据或者到达边界
        // 看起很高级，其实最坏情况还是O(n)，即是全部都是一样的数一样
        // Passed 0ms 2.3mb
        // match nums.binary_search(&target) {
        //     Ok(i) => {
        //         let (mut count, mut le, mut ri) = (1, i, i + 1);
        //         while le > 0 {
        //             le -= 1;
        //             if nums[le] == target { count += 1; } else { break; }
        //         }
        //         while ri < nums.len() && nums[ri] == target {
        //             count += 1;
        //             ri += 1;
        //         }
        //         count
        //     }
        //     Err(_) => 0
        // }

        // 方法4
        // 利用有序特性，改造二分查找，利用二分查找分别找到target的左右边界，left和right，那么结果就是right - left + 1
        // 将le和ri记为(0, len - 1)，
        // 计算m = (le + ri) / 2
        // 如果 nums[m] < target，则说明，target的边界在[m + 1, ri]中
        // 如果 nums[m] > target，则说明，target的边界在[le, m - 1]中
        // 如果 nums[m] == target，那么边界在左右都有可能，
        //     使用[m + 1, ri]找到右边界
        //     使用[le, m - 1]找到左边界
        // 我们简化一下思路，我们可以直接找出当前数字的右边界和，比他小一个数字的右边界，直接两个边界相减即可
        Self::upper_bound(&nums, target) - Self::upper_bound(&nums, target - 1)
    }

    fn upper_bound(nums: &Vec<i32>, target: i32) -> i32 {
        let (mut le, mut ri) = (0, nums.len() as i32 - 1);
        while le <= ri {
            let m = (le + ri) / 2;
            if nums[m as usize] <= target { le = m + 1; } else { ri = m - 1; }
        }
        le
    }
}

#[test]
fn test() {
    assert_eq!(Solution::search(vec![], 0), 0);
    assert_eq!(Solution::search(vec![1], 1), 1);
    assert_eq!(Solution::search(vec![5, 7, 7, 8, 8, 10], 8), 2);
    assert_eq!(Solution::search(vec![5, 7, 7, 8, 8, 10], 6), 0);
}