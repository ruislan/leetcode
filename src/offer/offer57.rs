use crate::offer::Solution;

#[allow(unused)]
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // 方法1
        // 注意是一个递增的数组，所以我们可以采用两边逼近的方式，
        // 左右两个指针，令le = 0，ri = nums.len() - 1，
        // 当 le < ri的时候，
        // 如果nums[ri] > target，那么ri -= 1，继续循环
        // 如果nums[ri] <= target，
        // 如果nums[le] + nums[ri] > target，说明ri还是很大，那么ri -= 1，继续循环
        // 否则如果nums[le] + nums[ri] < target，说明le还比较小，那么le += 1，继续循环
        // 否则nums[le] + nums[ri] == target，那么就找到了这两个数字，返回这两个数
        // 当le >= ri，循环结束，如果都没找到，那么返回空数组
        // Passed 24ms 3.5mb
        // let (mut le, mut ri) = (0, nums.len().saturating_sub(1));
        // while le < ri {
        //     if nums[ri] > target {
        //         ri -= 1;
        //     } else {
        //         if nums[le] + nums[ri] > target {
        //             ri -= 1;
        //         } else if nums[le] + nums[ri] < target {
        //             le += 1;
        //         } else {
        //             return vec![nums[le], nums[ri]];
        //         }
        //     }
        // }
        // vec![]

        // 方法2
        // 方法1的改进版本，不用先去找小于target的数据，直接两边相加，
        // 当大于target，则ri -= 1
        // 当小于target，则le += 1
        // 等于target，返回
        // Passed 24ms 3.5mb
        // let (mut le, mut ri) = (0, nums.len().saturating_sub(1));
        // while le < ri {
        //     if nums[le] + nums[ri] > target {
        //         ri -= 1;
        //     } else if nums[le] + nums[ri] < target {
        //         le += 1;
        //     } else {
        //         return vec![nums[le], nums[ri]];
        //     }
        // }
        // vec![]

        // 方法3
        // 建造一个hashset，
        // 然后我们迭代nums，将所有小于target的数字放入hashmap
        // 放入hashmap之前，首先检查是否存在target - cur_num的数字是否在hashmap中
        // 如果存在，则直接返回cur_num和hashmap中的对应数字
        // 迭代一直到nums[i] > target 或者到端点为止
        // Passed 68ms 4.3mb
        // let mut sets = std::collections::HashSet::new();
        // for n in nums {
        //     if n > target { break; }
        //     if sets.contains(&(target - n)) {
        //         return vec![target - n, n];
        //     } else {
        //         sets.insert(n);
        //     }
        // }
        // vec![]

        // 方法4
        // 方法3的改进版本，因为rust的hash的计算策略的并不是最高效率的，所以我们采用数组来保存
        // 因为我们最高也就是target，所以我们建设一个target长度的数组，这样从0到target的数字都能包括进去
        // Passed 32ms 9.6mb
        // let mut arr = vec![0; target as usize];
        // nums.iter().filter(|&&x| x < target).for_each(|&x| arr[x as usize] += 1);
        // nums.iter().find(|&&x| arr[(target - x) as usize] > 0).map_or(vec![], |&x| vec![x, target - x])

        // 方法5
        // 方法4的改进版本，我们迭代都是从最小数字开始，所以刚开始都是往数组里填数字
        // 然后检查他的对向数字是否存在，如果存在就返回这两个数字，直到结束或者大于了target
        // Passed 12ms 9.5mb
        let mut arr = vec![0; target as usize];
        for &x in nums.iter().filter(|&&x| x < target)  {
            if arr[(target - x) as usize] > 0 {
                return vec![target - x, x];
            } else {
                arr[x as usize] += 1;
            }
        }
        vec![]
    }
}

#[test]
fn test() {
    assert_eq!(Solution::two_sum(vec![], 0), vec![]);
    assert_eq!(Solution::two_sum(vec![1], 1), vec![]);
    assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![2, 7]);
    assert_eq!(Solution::two_sum(vec![10, 26, 30, 31, 47, 60], 40), vec![10, 30]);
}