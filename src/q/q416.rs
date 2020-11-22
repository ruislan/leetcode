use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        // 特点：正整数，非零数组，能划分成两个数组，
        // 意味着：和只能变大，不考虑没有数字的情况，数组总和能被2整除
        // 那么，我们可以转换问题为
        // 从数组中找出几个数字刚刚好能够等于数组总和的一半
        // 方法1，
        // 暴力求解，
        // 那么意味着第一次选择n-1个数字，有n种选择方案
        // 第二次选择n-2个数字，而有n-1种选择方案
        // ……
        // 所以这个方案时间为O(n^3)，一定超时
        // 方法2
        // 动态规划，怎么想到用动态规划呢？这个需要多做动态规划的题才可以
        // 动态规划的特点就是能够先求解子问题的解，然后合并子问题的解得到更大问题的解，
        // 那么这题正好适合，因为我们可以转换成一个背包问题
        // 总和的一半刚好就是背包的总重量，每个数字恰好是每个物品的重量，
        // 那么问题就是选取那几个物品（数字）的重量正好等于背包的总重量
        // 而一个背包问题那就是典型的动态规划问题了。
        // 动态规划首先画出一个二维数组，那么row和col如何选择？
        // 背包问题中，col是逐步从1到n的重量,n是背包能够装的总重量，row是每个物品的重量
        // 这里，我们映射过来就是col就是逐步从1到sum/2的数字，row是数组中的每个数字
        // 例如： 1 5 11 5， sum = 22, sum / 2 = 11
        // col/row   0  1  2  3  4  5  6  7  8  9  10 11
        //      1    0  1  1  1  1  1  1  1  1  1  1  1
        //      5    0  1  1  1  1  5  6  6  6  6  6  6
        //     11    0  1  1  1  1  5  6  6  6  6  6  11
        //      5    0  1  1  1  1  5  6  6  6  6  10 11
        // 从上述我们可以看到
        // 我们只需要确认最右下这个位置dp[nums.len() - 1][target + 1] == target即可
        // 那么我们怎么确定每个dp[row][col]的值，
        // 首先，第一行，只需要判断col >= nums[0]的都能存下即可。
        // 然后，
        // 如果，col的值大于当前数字nums[row]，那么意味着可以装下当前数字，那么装下当前数字之后，剩下的格数就去找能装下剩下格数的那个数字,然后比较谁大就放谁
        // 如果，col的值小于当前数字nums[row]，那么意味着不能装下当前数字，那么就装下dp[row - 1][col]的数字
        // 即是，
        // dp[row][col] = (dp[row - 1][col - nums[row]] + nums[row]).max(dp[row - 1][col - nums[row]])，col >= nums[row]
        // dp[row][col] = dp[row - 1][col], col < nums[i]
        // 算出了每个dp[row][col]的值之后，
        // 返回dp[nums.len() - 1][target + 1] == target 即可
        // Passed 42ms 17.1mb
        // let sum = nums.iter().sum::<i32>();
        // if sum & 1 == 1 { return false; }
        // let target = sum as usize >> 1;
        // let mut dp = vec![vec![0; target + 1]; nums.len()];
        // for col in 1..target + 1 {
        //     if col >= nums[0] as usize { dp[0][col] = nums[0] as usize; }
        // }
        // for row in 1..nums.len() {
        //     for col in 1..target + 1 {
        //         if col >= nums[row] as usize {
        //             dp[row][col] = dp[row - 1][col].max(dp[row - 1][col - nums[row] as usize] + nums[row] as usize)
        //         }
        //         // 可以不用计算左下角，这样减少一半的计算量
        //         // else {
        //         //     dp[row][col] = dp[row - 1][col]
        //         // }
        //     }
        // }
        // dp[nums.len() - 1][target] == target

        // 方法2
        // 我们来优化一下，其实不用二维数组，压缩到一行就可以解决
        // 因为这一行始终记录最新的状态，然后看最后一个是否等于target即可
        // 例如： 1 5 11 5， sum = 22, target = 11
        // col/row   0  1  2  3  4  5  6  7  8  9  10 11
        //      1    0  1  1  1  1  1  1  1  1  1  1  1
        //      5    0  1  1  1  1  5  6  6  6  6  6  6
        //     11    0  1  1  1  1  5  6  6  6  6  6  11
        //      5    0  1  1  1  1  5  6  6  6  6  10 11
        // 需要注意的是，在计算每个dp[col]的时候需要倒着往前计算
        // 因为正着计算会导致后面的值会用到前面已经改变了状态的值，
        // 而倒着往前算的时候，前面的值都是未改变状态的值。
        // Passed 28ms 2.1mb
        // let sum = nums.iter().sum::<i32>();
        // if sum & 1 == 1 { return false; }
        // let target = sum >> 1;
        // let mut dp = vec![0; target as usize + 1];
        // for n in nums {
        //     for col in (n as usize..=target as usize).rev() {
        //         dp[col] = dp[col].max(n + dp[col - n as usize]);
        //     }
        // }
        // dp[target as usize] == target

        // 方法3
        // 再次优化，我们不再计算数字，而采用bool，刚刚能装下的值为true，否则为false
        // 例如： 1 5 11 5， sum = 22, target = 11
        // col/row   0  1  2  3  4  5  6  7  8  9  10 11
        //      1    T  T  F  F  F  F  F  F  F  F  F  F
        //      5    T  T  F  F  F  T  T  F  F  F  F  F
        //     11    T  T  F  F  F  T  T  F  F  F  F  T
        //      5    T  T  F  F  F  T  T  F  F  F  T  T
        // Passed 28ms 2mb
        let sum = nums.iter().sum::<i32>();
        if sum & 1 == 1 { return false; }
        let target = sum >> 1;
        let mut dp = vec![false; target as usize + 1];
        dp[0] = true;
        for n in nums {
            for col in (n as usize..=target as usize).rev() {
                dp[col] = dp[col] | dp[col - n as usize]
            }
        }
        dp[target as usize]
    }
}

#[test]
fn test() {
    assert_eq!(Solution::can_partition(vec![1]), false);
    assert_eq!(Solution::can_partition(vec![1, 2]), false);
    assert_eq!(Solution::can_partition(vec![1, 2, 3]), true);
    assert_eq!(Solution::can_partition(vec![1, 2, 5]), false);
    assert_eq!(Solution::can_partition(vec![1, 2, 1]), true);
    assert_eq!(Solution::can_partition(vec![1, 4, 1, 2]), true);
    assert_eq!(Solution::can_partition(vec![2, 2, 1, 1]), true);
    assert_eq!(Solution::can_partition(vec![1, 2, 3, 4]), true); // [1,4] == [2,3]
    assert_eq!(Solution::can_partition(vec![1, 2, 5, 8]), true);
    assert_eq!(Solution::can_partition(vec![1, 5, 11, 5]), true);
    assert_eq!(Solution::can_partition(vec![1, 2, 3, 3, 3]), true);
}