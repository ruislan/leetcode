use crate::q::Solution;

impl Solution {
    pub fn min_subsequence(nums: Vec<i32>) -> Vec<i32> {
        // 方法1
        // 弄个数组arr长度101，然后迭代nums，将nums[i]作为arr下标，记录数字频率
        // 统计arr的total
        // 然后按照最大的下标（下标即数字）向前相加sum，如果sum < (total - sum)，将这个数字放入结果res
        // 如果sum > (total - sum)， 则退出，并返回结果res
        let mut arr = vec![0; 101];
        nums.iter().for_each(|&x| { arr[x as usize] += 1; });
        let arr: Vec<i32> = arr.iter().enumerate().filter(|&(_, &x)| x > 0).map(|(i, &x)| vec![i as i32; x as usize]).flatten().collect();
        let total: i32 = arr.iter().sum();
        let (mut sum, mut res) = (0, Vec::new());
        for &x in arr.iter().rev() {
            if sum > total - sum { break; }
            res.push(x);
            sum += x;
        }
        res


        // 方法2
        // 倒排序nums，求和nums为total
        // 迭代nums，当 sum < (total - sum)，则将这个数字放入结果res
        // 如果sum > (total - sum)，则退出，并返回结果res
        // Passed 0ms-4ms, 2.1mb
        // let mut nums = nums;
        // nums.sort_unstable_by(|a, b| b.cmp(a));
        // let total: i32 = nums.iter().sum();
        // let (mut sum, mut res) = (0, Vec::new());
        // for x in nums {
        //     if sum > total - sum { break; }
        //     res.push(x);
        //     sum += x;
        // }
        // res
    }
}

#[test]
fn test() {
    assert_eq!(Solution::min_subsequence(vec![4, 3, 10, 9, 8]), vec![10, 9]);
    assert_eq!(Solution::min_subsequence(vec![4, 4, 7, 6, 7]), vec![7, 7, 6]);
    assert_eq!(Solution::min_subsequence(vec![6]), vec![6]);
}