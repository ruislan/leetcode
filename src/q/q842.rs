use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn split_into_fibonacci(s: String) -> Vec<i32> {
        // 方法1
        // 不要看错了，是拆分，并非排列组合
        // 所以我们将字符串逐个进行拆分
        // 以下几种情况不能拆分：
        //    已经到末尾了
        //    数字比i32.MAX还要大
        //    数字比前两个数字的和要大
        // 以下几种情况得不到结果
        //    nums[i] + nums[i + 1] != nums[i + 2]
        //    nums.len() <= 2
        // 所以我们需要枚举所有的情况，如果此路不通，就要回到枚举的前一种情况继续探路，很像DFS
        // 这里我们需要一个数组来记录走过的路
        // Passed 0ms 2mb
        fn traverse(position: usize, s: &Vec<i32>, nums: &mut Vec<i32>, prev: i32, sum: i32) -> bool {
            if position == s.len() { return nums.len() > 2; }
            let mut x = 0_i64;
            for i in position..s.len() {
                if s[position] == 0 && i > position { break; }
                x = 10 * x + s[i] as i64;
                if x > i32::MAX as i64 { break; }

                let x = x as i32;
                if nums.len() >= 2 {
                    if x < sum { continue; }
                    if x > sum { break; }
                }
                nums.push(x);
                if traverse(i + 1, s, nums, x, x + prev) {
                    return true;
                } else {
                    nums.pop();
                }
            }
            false
        }

        let mut answer = Vec::new();
        let s: Vec<i32> = s.into_bytes().into_iter().map(|x| (x - '0' as u8) as i32).collect();
        traverse(0, &s, &mut answer, 0, 0);
        answer
    }
}

#[test]
fn test() {
    assert_eq!(Solution::split_into_fibonacci("0123".to_string()), vec![]);
    assert_eq!(Solution::split_into_fibonacci("123456579".to_string()), vec![123, 456, 579]);
}