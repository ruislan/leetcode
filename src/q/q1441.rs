use crate::q::Solution;

impl Solution {
    pub fn build_array(target: Vec<i32>, n: i32) -> Vec<String> {
        // 方法1
        // target.last() + 1长度的一个数组arr，按照target[i]作为下标设置值为1
        // 构建返回数组res，从下标1开始迭代arr
        //    如果arr[i] = 1并且i <= n， 放入"Push"
        //    如果arr[i] = 0或者i > n，放入"Push"和"Pop"
        // Passed 0ms 2.1mb
        let mut arr = vec![0; *target.last().unwrap_or(&0) as usize + 1];
        (0..target.len()).for_each(|i| arr[target[i] as usize] = 1);
        arr.into_iter()
            .skip(1)
            .map(|x| if x == 0 { vec!["Push".to_string(), "Pop".to_string()] } else { vec!["Push".to_string()] })
            .flatten()
            .collect()
    }
}

#[test]
fn test() {
    // assert_eq!(Solution::build_array(vec![], 0), vec![].into_iter().map(|x| x.to_string()).collect::<Vec<String>>());
    assert_eq!(Solution::build_array(vec![1, 3], 3), vec!["Push", "Push", "Pop", "Push"].into_iter().map(|x| x.to_string()).collect::<Vec<String>>());
    assert_eq!(Solution::build_array(vec![1, 2, 3], 3), vec!["Push", "Push", "Push"].into_iter().map(|x| x.to_string()).collect::<Vec<String>>());
    assert_eq!(Solution::build_array(vec![1, 2], 4), vec!["Push", "Push"].into_iter().map(|x| x.to_string()).collect::<Vec<String>>());
    assert_eq!(Solution::build_array(vec![2, 3, 4], 4), vec!["Push", "Pop", "Push", "Push", "Push"].into_iter().map(|x| x.to_string()).collect::<Vec<String>>());
    // 我觉得下面这个答案应该是"Push","Pop"，因为每次是从1..n中依序选一个，现在n=1，也就是说第一次就选完了，第二次没有数字可以选，所以没有操作指令
    // 但是leetcode编辑器的结果是"Push", "Pop", "Push"
    assert_eq!(Solution::build_array(vec![2], 1), vec!["Push", "Pop", "Push"].into_iter().map(|x| x.to_string()).collect::<Vec<String>>());
}