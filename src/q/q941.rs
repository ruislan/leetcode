use crate::q::Solution;

impl Solution {
    pub fn valid_mountain_array(a: Vec<i32>) -> bool {
        // 方法1
        // Passed 4ms 2mb
        // if a.len() < 3 { return false; }
        // let mut top = None;
        // let mut up = 0;
        // let mut down = 0;
        // for i in 1..a.len() {
        //     if top == None {
        //         if a[i - 1] < a[i] {
        //             up += 1;
        //         } else if a[i - 1] > a[i] {
        //             top = Some(i - 1);
        //             down += 1;
        //         } else {
        //             return false;
        //         }
        //     } else {
        //         if up == 0 { return false; }
        //         if a[i - 1] > a[i] {
        //             down += 1;
        //         } else {
        //             return false;
        //         }
        //     }
        // }
        // if top == None || up == 0 || down == 0 { return false; }
        // return true;

        // 方法2
        // 今天每日一题到这里了，看到方法1有点复杂，优化一下
        // 两个方法
        // 方法2.1
        // 先找到数组最大点top，然后检查top之前的位置是否是连续升（包含top），top之后的点是否是连续降（包含top）
        // Passed 0ms 2mb
        // if a.len() < 3 { return false; }
        // let top = a.iter().enumerate().max_by(|x, y| (*x).1.cmp((*y).1)).unwrap().0;
        // if top == 0 || top == a.len() - 1 { return false; }
        // for i in 1..=top {
        //     if a[i - 1] >= a[i] { return false; }
        // }
        // for i in (top + 1)..a.len() {
        //     if a[i - 1] <= a[i] { return false; }
        // }
        // return true;

        // 方法2.2
        // 一次扫描
        // 比较i与i+1，当a[i] >= a[i+1]，如果i是第一个或者最后一个，返回false
        // 比较i与i+1，当a[i] <= a[i+1]，返回false
        // 返回true
        // Passed 0ms 2.1mb
        if a.len() < 3 { return false; }
        let mut i = 0;
        while i < a.len() - 1 {
            if a[i] >= a[i + 1] { break; }
            i += 1;
        }
        if i == 0 || i == a.len() - 1 { return false; }
        while i < a.len() - 1 {
            if a[i] <= a[i + 1] { return false; }
            i += 1;
        }
        return true;
    }
}

#[test]
fn test() {
    assert_eq!(Solution::valid_mountain_array(vec![]), false);
    assert_eq!(Solution::valid_mountain_array(vec![3, 5, 5]), false);
    assert_eq!(Solution::valid_mountain_array(vec![0, 3, 2, 1]), true);
    assert_eq!(Solution::valid_mountain_array(vec![0, 1, 2, 3]), false);
    assert_eq!(Solution::valid_mountain_array(vec![0, 1, 2, 3, 0]), true);
    assert_eq!(Solution::valid_mountain_array(vec![3, 2, 1, 0]), false);
    assert_eq!(Solution::valid_mountain_array(vec![3, 2, 1, 0, 1, 2, 3]), false);
}