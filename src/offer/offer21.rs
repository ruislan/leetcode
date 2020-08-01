use crate::offer::Solution;

impl Solution {
    pub fn exchange(nums: Vec<i32>) -> Vec<i32> {
        // 方法1
        // 创建两个数组odd和even，迭代nums
        // 如果nums[i]是奇数push进odd，如果是偶数push进even
        // 然后将even数组append到odd数组后面即可
        // Passed 8ms 2.5mb
        // let mut odds = nums.iter().filter(|&&x| x & 1 == 1).map(|&x| x).collect::<Vec<i32>>();
        // let mut evens = nums.iter().filter(|&&x| x & 1 == 0).map(|&x| x).collect::<Vec<i32>>();
        // odds.append(&mut evens);
        // odds

        // 方法2
        // 采用双指针le和ri
        // while当le小于ri，循环le找到偶数，循环ri找到奇数，
        // 如果le < ri，交换le和ri的数，使得le += 1， ri -= 1
        // 返回nums
        // Passed 8ms 2.5mb
        let mut nums = nums;
        let mut le = 0;
        let mut ri = if nums.is_empty() { 0 } else { nums.len() - 1 };
        while le < ri {
            if nums[le] & 1 != 0 {
                le += 1;
                continue;
            }
            if nums[ri] & 1 != 1 {
                ri -= 1;
                continue;
            }
            nums.swap(le, ri);
        }
        nums
    }
}

#[test]
fn test() {
    // 1 <= nums.length <= 50000
    // 1 <= nums[i] <= 10000
    assert_eq!(Solution::exchange(vec![]), vec![]);
    assert_eq!(Solution::exchange(vec![1]), vec![1]);
    assert_eq!(Solution::exchange(vec![1, 2]), vec![1, 2]);
    assert_eq!(Solution::exchange(vec![2, 1]), vec![1, 2]);
    assert_eq!(Solution::exchange(vec![1, 2, 3, 4]), vec![1, 3, 2, 4]);
    assert_eq!(Solution::exchange(vec![1, 1, 1, 1]), vec![1, 1, 1, 1]);
    assert_eq!(Solution::exchange(vec![2, 2, 2, 2]), vec![2, 2, 2, 2]);
}