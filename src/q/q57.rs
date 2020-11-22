use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        // 方法1
        // 首先将new_interval加入intervals，排序intervals
        //
        // 说明一下：Rust的排序很快，所以直接排序了，所以这里是O(nlogn)，
        //          其实也可以不用排序，逐步找到插入位置，就可以改成O(n)。
        //
        // 设结果集为answer
        // 迭代intervals：
        //    检查answer.last(last)与intervals[i](x)：
        //        如果x[0] > last[1]：
        //           则将x插入answer
        //        否则：
        //           如果 x[1] > last[1]：
        //              设置last[1] = x[1]
        // Passed 0ms 2.6mb
        let mut intervals = intervals;
        intervals.push(new_interval);
        intervals.sort_unstable();
        let mut answer = Vec::new();
        for x in intervals {
            match answer.last_mut() {
                None => answer.push(x),
                Some(last) => {
                    if x[0] > last[1] {
                        answer.push(x);
                    } else {
                        last[1] = last[1].max(x[1]);
                    }
                }
            }
        }
        answer
    }
}

#[test]
fn test() {
    assert_eq!(Solution::insert(vec![vec![1, 5]], vec![0, 0]), vec![vec![0, 0], vec![1, 5]]);
    assert_eq!(Solution::insert(vec![vec![1, 5]], vec![6, 6]), vec![vec![1, 5], vec![6, 6]]);
    assert_eq!(Solution::insert(vec![vec![1, 7]], vec![1, 5]), vec![vec![1, 7]]);
    assert_eq!(Solution::insert(vec![vec![1, 5]], vec![1, 7]), vec![vec![1, 7]]);
    assert_eq!(Solution::insert(vec![vec![1, 3], vec![6, 9]], vec![2, 5]), vec![vec![1, 5], vec![6, 9]]);
    assert_eq!(Solution::insert(vec![vec![1, 2], vec![3, 5], vec![6, 7], vec![8, 10], vec![12, 16]], vec![4, 8]), vec![vec![1, 2], vec![3, 10], vec![12, 16]]);
}