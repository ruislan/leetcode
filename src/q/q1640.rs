use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn can_form_array(arr: Vec<i32>, pieces: Vec<Vec<i32>>) -> bool {
        // 方法1
        // 如果pieces中的每个piece都是arr的子数组，那么就可以拼接
        // 嗯，没有找到检查是不是该数组子数组的库函数，要不自己弄一个？
        // Passed 0ms 2mb
        for piece in pieces {
            match arr.iter().position(|&x| x == piece[0]) {
                None => return false,
                Some(mut i) => {
                    let mut j = 1;
                    i += 1;
                    while i < arr.len() && j < piece.len() {
                        if arr[i] != piece[j] { return false; }
                        i += 1;
                        j += 1;
                    }
                    if j != piece.len() { return false; }
                }
            }
        }
        true
    }
}

#[test]
fn test() {
    assert_eq!(Solution::can_form_array(vec![85], vec![vec![85]]), true);
    assert_eq!(Solution::can_form_array(vec![15, 88], vec![vec![88], vec![15]]), true);
    assert_eq!(Solution::can_form_array(vec![49, 18, 16], vec![vec![16, 18, 49]]), false);
    assert_eq!(Solution::can_form_array(vec![91, 4, 64, 78], vec![vec![78], vec![4, 64], vec![91]]), true);
    assert_eq!(Solution::can_form_array(vec![1, 3, 5, 7], vec![vec![2, 4, 6, 8]]), false);
    assert_eq!(Solution::can_form_array(vec![1, 3, 5, 7], vec![vec![1, 3, 5, 8]]), false);
}