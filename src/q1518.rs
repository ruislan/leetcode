use crate::Solution;

impl Solution {
    pub fn num_water_bottles(num_bottles: i32, num_exchange: i32) -> i32 {
        // 方法1
        // 创建一个喝酒计数器等于num_bottles，迭代while num_bottles >= num_exchange，大于等于num_exchange是因为最后一轮少于这个数就不能兑换一瓶新的了
        // count += num_bottles / num_exchange ，num_bottles = num_bottles / num_exchange + num_bottles % num_exchange
        // 返回count
        // 例1：9， 3，count初始为9，然后count += 9/3 + 9%3后，count为12，num_bottles为3，然后count += 3/3为13，num_bottles为1，小于3，不用迭代了
        // 例2: 15，4，count初始15，然后count += 15/4后，count为3+15为18，n_b为3+(15%4)为6，然后count += 6/4后，count为18+1=9，n_b为1+(6%4)，小于4不能再兑换了
        0
    }
}