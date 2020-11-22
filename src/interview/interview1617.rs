use crate::interview::Solution;

#[allow(unused)]
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        // 方法1
        // 典型的动态规划题
        // 初始化最大值是i32最小值
        // 然后用一个sum来记录当前的子数组的和（这个和可能是最大，也可能不是）
        // 然后迭代nums
        // 计算当前的和sum + n，那我们是否接受这个和呢？
        // 关键就看这个和是否比当的nums[i]大：
        //    如果大就接受 sum = sum + n
        //    如果nums[i]比较大，那前面的sum反而拖后腿了，sum = nums[i]
        // 当得到这最新的sum之后，与max比较：
        //    如果max较大：那就不变
        //    如果sum较大，那就max = sum
        // 这样保证了最后的max就是最大的连续子数组的和
        let mut max = i32::MIN;
        let mut sum = 0;
        for n in nums {
            sum = n.max(sum + n);
            max = max.max(sum);
        }
        max
    }
}