use crate::q::Solution;

impl Solution {
    pub fn sort_array_by_parity_ii(a: Vec<i32>) -> Vec<i32> {
        // 方法1
        // 一个数组存奇数，一个数组存偶数
        // 然后依次放入结果
        // Passed 12ms 2mb
        // let mut even = Vec::new();
        // let mut odd = Vec::new();
        //
        // for i in 0..a.len() {
        //     if a[i] % 2 == 0 {
        //         even.push(a[i]);
        //     } else {
        //         odd.push(a[i]);
        //     }
        // }
        // let mut res = Vec::new();
        // for i in 0..odd.len() {
        //     res.push(even.pop().unwrap());
        //     res.push(odd.pop().unwrap());
        // }
        // res


        // 方法2
        // 用一个额外的数组存储结果
        // 迭代a[i]，
        //    当时偶数时，放入偶数位置，偶数索引+2
        //    当是奇数时，放入奇数位置，奇数索引+2
        // Passed 8ms 2.1mb
        // let mut answer = vec![0; a.len()];
        // let (mut even, mut odd) = (0, 1);
        // for i in 0..a.len() {
        //     if a[i] & 1 == 0 {
        //         answer[even] = a[i];
        //         even += 2;
        //     } else {
        //         answer[odd] = a[i];
        //         odd += 2;
        //     }
        // }
        // answer

        // 方法3
        // 双指针，原地交换，不需要额外空间
        // 从左向右找偶数，从右向左找奇数（或者两个都从起点找都没问题）
        // 找到之后交换
        // 当左边到达终点或者右边到达起点，完成交换
        // Passed 12ms 2.2mb
        let mut a = a;
        let len = a.len() as i32;
        let (mut lo, mut hi) = (0_i32, if len & 1 == 1 { len - 2 } else { len - 1 });
        while lo < len && hi >= 0 {
            while lo < len && a[lo as usize] & 1 == 0 { lo += 2; }
            while hi >= 0 && a[hi as usize] & 1 == 1 { hi -= 2; }
            if lo < len && hi >= 0 { a.swap(lo as usize, hi as usize); }
        }
        a
    }
}

#[test]
fn test() {
    assert_eq!(Solution::sort_array_by_parity_ii(vec![1, 2]), vec![2, 1]);
    assert_eq!(Solution::sort_array_by_parity_ii(vec![2, 1]), vec![2, 1]);
    let answer = Solution::sort_array_by_parity_ii(vec![4, 2, 5, 7]);
    assert_eq!(true, vec![vec![4, 5, 2, 7], vec![2, 5, 4, 7], vec![2, 7, 4, 5]].into_iter().any(|x| x == answer));
}