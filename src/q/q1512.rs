use crate::q::Solution;

impl Solution {
    pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        // 方法1
        // 创建计数器count = 0,
        // 创建一个101长度的数组arr,初始值为0，迭代nums
        // 以nums[i]为索引，将里面的值加到count上，count += arr[nums[i]]
        // 然后将arr[nums[i]]增加1
        // 一直到迭代结束，返回count即是好数对的结果
        let (mut count, mut arr) = (0, vec![0; 101]);
        nums.iter().for_each(|&x| {
            count += arr[x as usize];
            arr[x as usize] += 1;
        });
        count
    }
}

#[test]
fn test() {
    // assert_eq!(Solution::num_identical_pairs(vec![]), 0);
    assert_eq!(Solution::num_identical_pairs(vec![1, 2, 3, 1, 1, 3]), 4);
    assert_eq!(Solution::num_identical_pairs(vec![1, 1, 1, 1]), 6);
    assert_eq!(Solution::num_identical_pairs(vec![1, 2, 3]), 0);
}