use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn maximum_time(time: String) -> String {
        // 方法1
        // 看第一位数字：
        //    如果第一位是？，那么最高是2，
        //       如果第二位是？或者小于3，那么继续
        //       否则，最高是1
        //    否则继续
        // 看第二位数字
        //    如果第一位是2，那么最高是3，
        //     否则，最高是9
        // 看第三位数字，如果是？，那么最高是5
        // 看第四位数字，如果是？，那么最高是9
        // AC 0ms 2.1mb
        let mut nums = time.chars().collect::<Vec<char>>();
        if nums[0] == '?' {
            if nums[1] == '?' || nums[1] <= '3' {
                nums[0] = '2';
            } else {
                nums[0] = '1';
            }
        }

        if nums[1] == '?' {
            if nums[0] == '2' {
                nums[1] = '3';
            } else {
                nums[1] = '9';
            }
        }
        // nums[2] = ':'
        if nums[3] == '?' { nums[3] = '5'; }
        if nums[4] == '?' { nums[4] = '9'; }

        nums.into_iter().collect()
    }
}
// 2?:?0
// 09:39

// 0[9]:[5][9]
// 1[9]
// 2[3]
