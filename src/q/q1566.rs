use crate::q::Solution;

impl Solution {
    pub fn contains_pattern(arr: Vec<i32>, m: i32, k: i32) -> bool {
        // 方法1
        // 按照0..m，1..m+1，2..m+2直到，n..m+n，m+n < arr.len()的方式迭代
        // 创建k个指针，每个指针起点间隔m，k1=0,k2=m,k3=2m,k4=3m...
        // 同时移动所有的指针，长度为m-1，如果每个step都相等，则返回true
        // 如果不相等，则k1=1,k2=m+1,k3=2m+1,k4=3m+1
        // 重复上述，直到第k个的指针+m大于等于len
        // 迭代完成返回false
        // Passed 0ms 2.1mb
        // let (m, k, mut ptrs) = (m as usize, k as usize, Vec::new());
        // for i in 0..k { ptrs.push(i * m); }
        // while *ptrs.last().unwrap() + m <= arr.len() {
        //     let mut find = true;
        //     for i in 0..m {
        //         for j in 1..ptrs.len() {
        //             if arr[ptrs[j] + i] != arr[ptrs[j - 1] + i] {
        //                 find = false;
        //                 break;
        //             }
        //         }
        //     }
        //     if find { return true; }
        //     for i in 0..ptrs.len() { ptrs[i] += 1; }
        // }
        // false

        // 方法2
        // 利用库函数chunks
        // Passed 0ms 2.1mb
        let (m, k) = (m as usize, k as usize);
        arr.windows(m * k).any(|window| window.chunks(m).all(|chunk| chunk == &window[0..m]))
    }
}

#[test]
fn test() {
    // 2 <= arr.length <= 100
    // 1 <= arr[i] <= 100
    // 1 <= m <= 100
    // 2 <= k <= 100
    assert_eq!(Solution::contains_pattern(vec![1, 1], 1, 2), true);
    assert_eq!(Solution::contains_pattern(vec![1, 2, 4, 4, 4, 4], 1, 3), true);
    assert_eq!(Solution::contains_pattern(vec![1, 2, 1, 2, 1, 1, 1, 3], 2, 2), true);
    assert_eq!(Solution::contains_pattern(vec![1, 2, 1, 2, 1, 3], 2, 3), false);
    assert_eq!(Solution::contains_pattern(vec![1, 2, 3, 1, 2], 2, 2), false);
    assert_eq!(Solution::contains_pattern(vec![2, 2, 2, 2], 2, 3), false);
}