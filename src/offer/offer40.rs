use crate::offer::Solution;

#[allow(unused)]
impl Solution {
    pub fn get_least_numbers(arr: Vec<i32>, k: i32) -> Vec<i32> {
        // 方法1
        // 直接排序，取前k个数字即可
        // Passed 8ms 2.1mb
        // let mut arr = arr;
        // arr.sort_unstable();
        // arr.into_iter().take(k as usize).collect()

        // 方法2
        // 堆，取堆顶的k个即可
        // Passed 12ms 2.3mb
        // let mut heap: std::collections::BinaryHeap<std::cmp::Reverse<i32>> = arr.into_iter().map(|x| std::cmp::Reverse(x)).collect();
        // (0..k).map(|i| heap.pop().unwrap().0).collect()

        // 方法3
        // 创建一个10001的数组，arr[i]放入数组，然后迭代arr，取k个不等于0的
        // Passed 8ms 2.1mb
        let mut v = vec![0; 10001];
        arr.into_iter().for_each(|x| v[x as usize] += 1);
        let mut res = Vec::new();
        for i in 0..10001 {
            while v[i] > 0 && res.len() < k as usize {
                res.push(i as i32);
                v[i] -= 1;
            }
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(Solution::get_least_numbers(vec![], 0), vec![]);
    assert_eq!(Solution::get_least_numbers(vec![1], 1), vec![1]);
    assert_eq!(Solution::get_least_numbers(vec![3, 2, 1], 2), vec![1, 2]);
    assert_eq!(Solution::get_least_numbers(vec![0, 1, 2, 1], 1), vec![0]);
}