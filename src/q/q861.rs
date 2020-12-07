use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn matrix_score(a: Vec<Vec<i32>>) -> i32 {
        // 方法1
        // 首先进行横向修订
        //    如果第一个不为0，就直接翻转
        // 再进行纵向修订
        //    如果0比1多，就翻转
        // Passed 0ms 1.9mb
        let mut a = a;
        let rows = a.len();
        let cols = a[0].len();

        for row in 0..rows {
            if a[row][0] == 0 {
                for col in 0..cols {
                    a[row][col] = 1 - a[row][col];
                }
            }
        }

        let half_rows = rows / 2;
        for col in 0..cols {
            let mut zeros = 0;
            for row in 0..rows {
                if a[row][col] == 0 { zeros += 1; }
            }

            if zeros > half_rows {
                for row in 0..rows {
                    a[row][col] = 1 - a[row][col];
                }
            }
        }
        
        let mut answer = 0;
        for row in 0..rows {
            let mut x = 0;
            for col in 0..cols {
                x = (x << 1) | a[row][col];
            }
            answer += x;
        }
        answer
    }
}

#[test]
fn test() {
    assert_eq!(Solution::matrix_score(vec![vec![0, 0, 1, 1], vec![1, 0, 1, 0], vec![1, 1, 0, 0]]), 39);
}