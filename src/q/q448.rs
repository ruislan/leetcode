use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        // 方法1
        // 记录数字的频率，然后统计没有频率的即可
        // AC 12ms 2.6mb
        // let n = nums.len();
        // let mut freq = vec![0; n];
        // for num in nums { freq[num as usize - 1] += 1; }
        // let mut answer = Vec::new();
        // for i in 0..n {
        //     if freq[i] == 0 {
        //         answer.push(i as i32 + 1);
        //     }
        // }
        // answer

        // 方法2
        // 要求不用额外的空间，也即是不用freq来记录
        // 那么我们只能在原数组上想办法
        // 想一下，假设我们所有的数字都加上n，
        // 这样的话，是不是缺少的数字必然小于等于n
        // 而重复的数字，无外乎就是大于2N，但是我们不关注这个
        // 我们只关注小于等于N的
        // 那么如何实现呢，因为数字是1-N，
        // 所以我们把每个数字x - 1作为下标来作为哪个数字应该加上n
        // AC 12ms 2.6mb
        let n = nums.len();
        let mut nums = nums;
        for i in 0..n {
            let x = (nums[i] as usize - 1) % n;
            nums[x] += n as i32;
        }
        let mut answer = Vec::new();
        for i in 0..n {
            if nums[i] <= n as i32{
                answer.push(i as i32 + 1);
            }
        }
        answer
    }
}