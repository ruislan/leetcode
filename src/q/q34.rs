use crate::q::Solution;

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // 方法1
        // 有序数组，二分搜索上界和下界
        // 需要对二分搜索变形
        // 二分搜索找到的那个数x，可能左边还有x
        // 所以，我们应该是找到nums[i] >= x，都向左搜索，这样可以找到下界，取低端的索引值
        // 上界的找法，可以用下界的反向操作，用nums[i] <= x，都向左搜索，取高端的索引值
        // 最后如果下界大于等于上界，那么返回[-1,-1]，否则返回[下界，上界]
        // rust 要多注意的地方就是索引只能是usize，所以有overflow的问题要处理
        // Passed 0ms 2.3mb
        // let (mut lo, mut hi) = (0, nums.len() - 1);
        // while lo <= hi && hi < nums.len() {
        //     let mid = lo + (hi - lo) / 2;
        //     if nums[mid] >= target {
        //         hi = mid.overflowing_sub(1).0;
        //     } else {
        //         lo = mid + 1;
        //     }
        // }
        // let low_bound = lo as i32;
        //
        // let (mut lo, mut hi) = (0, nums.len() - 1);
        // while lo <= hi && hi < nums.len() {
        //     let mid = lo + (hi - lo) / 2;
        //     if nums[mid] > target {
        //         hi = mid.overflowing_sub(1).0;
        //     } else {
        //         lo = mid + 1;
        //     }
        // }
        // let high_bound = hi as i32;
        //
        // if low_bound <= high_bound { vec![low_bound, high_bound] } else { vec![-1, -1] }

        // 方法2
        // 方法1还可以在找上界的时候再换个思路
        // 我们能够找到x的下界，那么我们完全可以也找到x + 1的下界
        // 这样结果不就是[x的下界， x+1的下界 - 1]了吗
        // 所以，我们就可以只需要一个找下界的函数即可处理
        // Passed 0ms 2.2mb
        fn find_low_bound(nums: &Vec<i32>, x: i32) -> usize {
            let (mut lo, mut hi) = (0, nums.len() - 1);
            while lo <= hi && hi < nums.len() {
                let mid = lo + (hi - lo) / 2;
                if nums[mid] >= x {
                    hi = mid.overflowing_sub(1).0;
                } else {
                    lo = mid + 1;
                }
            }
            lo
        }
        let low_bound = find_low_bound(&nums, target) as i32;
        let high_bound = find_low_bound(&nums, target + 1) as i32 - 1;
        if low_bound <= high_bound { vec![low_bound, high_bound] } else { vec![-1, -1] }
    }
}

#[test]
fn test() {
    assert_eq!(Solution::search_range(vec![5, 7, 7], 5), vec![0, 0]);
    assert_eq!(Solution::search_range(vec![5, 7, 7], 7), vec![1, 2]);
    assert_eq!(Solution::search_range(vec![5, 7, 7, 8, 8, 10], 8), vec![3, 4]);
    assert_eq!(Solution::search_range(vec![5, 7, 7, 8, 8, 10], 6), vec![-1, -1]);
}