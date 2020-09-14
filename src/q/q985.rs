use crate::q::Solution;

impl Solution {
    pub fn sum_even_after_queries(a: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        // 方法1，创建结果数组res，和当前偶数和sum，迭代queries，
        //  首先如果a[queries[i][1]]是偶数，那么sum先减去这个值
        //  然后看a[queries[i][1]] + queries[i][0]之后的值是否是偶数，是偶数就sum加上这个值
        //  存储sum
        let mut a = a;
        let mut res = Vec::with_capacity(queries.len());
        let mut sum = a.iter().filter(|&x| x & 1 == 0).sum();
        for i in 0..queries.len() {
            let index = queries[i][1] as usize;
            if a[index] & 1 == 0 { sum -= a[index]; }
            a[index] += queries[i][0];
            if a[index] & 1 == 0 { sum += a[index]; }
            res.push(sum);
        }
        res
    }
}

#[test]
fn test_q985() {
    assert_eq!(Solution::sum_even_after_queries(vec![1, 2, 3, 4], vec![vec![1, 0], vec![-3, 1], vec![-4, 0], vec![2, 3]]), vec![8, 6, 2, 4]);
}