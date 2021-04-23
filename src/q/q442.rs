use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn find_duplicates(nums: Vec<i32>) -> Vec<i32> {
        // 方法1
        // 用排序数组处理
        // AC 20ms 2.5mb
        // let mut nums = nums;
        // nums.sort_unstable();
        // let mut answer = Vec::new();
        // for i in 1..nums.len() {
        //     if nums[i] == nums[i - 1] {
        //         answer.push(nums[i]);
        //     }
        // }
        // answer

        // 方法2
        // 用HashSet判断
        // AC 28ms 3mb
        // let mut set = std::collections::HashSet::new();
        // let mut answer = Vec::new();
        // for &x in nums.iter() {
        //     if !set.insert(x) {
        //         answer.push(x);
        //     }
        // }
        // answer

        // 方法3
        // 因为数字是1..n，所以可以用数组替代HashSet
        // AC 16ms 2.6mb
        // let mut set = vec![0; nums.len()];
        // for &x in nums.iter() {
        //     set[x as usize - 1] += 1;
        // }
        // let mut answer = Vec::new();
        // for (i, &x) in set.iter().enumerate() {
        //     if x > 1 {
        //         answer.push(i as i32 + 1);
        //     }
        // }
        // answer

        // 方法4
        // 前面2个方法用了额外空间，第一额方法又是O(nlogn)的时间
        // 我们如果要用O(n)的时间，且没有额外的空间的话
        // 注意条件，是1..n个数字，n就是数组的长度
        // 也就是说，归位之后的数字，不在其位置上的一定就是重复的
        // 那么我们就不停的归位即可
        // 如果当前索引i的数字nums[i]不想等nums[i] - 1 != i
        // 那么意味着，我们正确的位置在j = nums[i] - 1上
        // 所以交换i, j，这个时候j索引的数字归位了
        // 而i的数字是新数字了，重复上述步骤，
        // 如果遇到要交换的数字nums[i]对应的j位置的数字已经归位了或者i本身就归位了
        // 那么就继续直到遍历完所有的数字
        // 这是一个O(n)的解法，且无需额外的空间
        // 最差的情况也就是每次换过来的数字刚好不在当前位置上，也就是迭代2n次
        // AC 16ms 2.6mb
        // let mut nums = nums;
        // let mut i = 0;
        // while i < nums.len() {
        //     while nums[i] - 1 != i as i32 && nums[nums[i] as usize - 1] != nums[i] {
        //         let j = nums[i] as usize - 1;
        //         nums.swap(j, i);
        //     }
        //     i += 1;
        // }
        // let mut answer = Vec::new();
        // for i in 0..nums.len() {
        //     if i as i32 != nums[i] - 1 {
        //         answer.push(nums[i]);
        //     }
        // }
        // answer

        // 方法5
        // 利用只出现两次和只有1..n个数字的特点
        // 将nums[i]位置j的数字设置为负数，负数用于表示这个位置已经归位了数字，
        // 这样我们遇到负数的数字时，就是出现过的数字，那么就加入到结果中
        // 例如  0 1 2 3 4 5 6 7
        //      4,3,2,7,8,2,3,1
        // 1：将nums[nums[0] - 1]的数字设置为-7， 4 3 2 -7 8 2 3 1
        // 2：将nums[nums[1] - 1]的数字设置为-2， 4 3 -2 -7 8 2 3 1
        // 3: 将nums[nums[2] - 1]的数字设置为-3， 4 -3 -2 -7 8 2 3 1
        // 4: 将nums[nums[3] - 1]的数字设置为-3， 4 -3 -2 -7 8 2 -3 1
        // 5: nums[nums[4] - 1] = -1， 4 -3 -2 -7 8 2 -3 -1
        // 6: nums[nums[5] - 1] 已经是-3了，那么意味着这个数是重复数
        // 7: nums[nums[6] - 1] 已经是-2了，那么意味着这个数是重复数
        // 8: nums[nums[7] - 1] = -4, -4 -3 -2 -7 8 2 -3 -1
        // 最后的数组是 -4 -3 -2 -7 8 2 -3 -1
        // 看出来了吧，“-”号就表示归位了的数字，而正数表示缺少的位置，这里正好就是缺（5，6）
        // 所以这个方法也能解决缺少了哪些数字
        // AC 20ms 2.7mb
        let mut nums = nums;
        let mut answer = Vec::new();
        for i in 0..nums.len() {
            let x = nums[i].abs() as usize;
            if nums[x - 1] > 0 {
                nums[x - 1] *= -1;
            } else {
                answer.push(x as i32);
            }
        }
        answer
    }
}
