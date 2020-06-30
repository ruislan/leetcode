struct Solution;

impl Solution {
    pub fn find_lucky(arr: Vec<i32>) -> i32 {
        // 方法1
        // 创建一个501长度的数组res，然后迭代arr，将arr[i]作为下标存储频率
        // 如果频率 == 下标，则返回最大的那个res.iter().enum..().filter(|n,i| i == n).max()
        let mut res = vec![0; 501];
        arr.iter().for_each(|&x| { res[x as usize] += 1; });
        (1..res.len()).filter(|&i| i as i32 == res[i])
            .map(|i| res[i])
            .max().unwrap_or(-1)
    }
}

#[test]
fn test() {
    // assert_eq!(Solution::find_lucky(vec![]),0);
    assert_eq!(Solution::find_lucky(vec![1]), 1);
    assert_eq!(Solution::find_lucky(vec![2, 2, 3, 4]), 2);
    assert_eq!(Solution::find_lucky(vec![1, 2, 2, 3, 3, 3]), 3);
    assert_eq!(Solution::find_lucky(vec![1, 2, 2, 3, 3, 3]), 3);
    assert_eq!(Solution::find_lucky(vec![2, 2, 2, 3, 3]), -1);
    assert_eq!(Solution::find_lucky(vec![5]), -1);
    assert_eq!(Solution::find_lucky(vec![7, 7, 7, 7, 7, 7, 7]), 7);
}