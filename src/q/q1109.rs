use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn corp_flight_bookings(bookings: Vec<Vec<i32>>, n: i32) -> Vec<i32> {
        // 方法1
        // 模拟法
        // O(n * m)
        // AC 980ms 3.7mb
        // let n = n as usize;
        // let mut answer = vec![0; n];
        // for booking in bookings {
        //     for i in booking[0]..=booking[1] {
        //         answer[i as usize] += booking[2];
        //     }
        // }
        // answer

        // 方法2
        // 差分数组
        // 差分就是数组中前后两个数字的差组成的数组
        // 例如: [1,2,2,4] => [1,1,0,2]
        // 我们对查分数组求前缀和就能求出原数组
        // 这里有我们遍历booking，然后求出区域内的差分，最后求出前缀和就可以了
        // 求出区域内的差分就是求出位置[lo - 1] += val 和[hi] -= val即可
        // 范例：[[1,2,10],[2,3,20],[2,5,25]]，n=5
        // 我们可以先一个一个的看：
        // [1,2,10] => [10,10]这个是累加后的，差分得到的是[10,0,-10]
        // [2,3,20] => [0,20,20]， 差分：[0,20,0,-20]
        // [2,5,25] => [0,25,25,25,25]，差分：[0,25,0,0,0]
        // 然后我们叠加一下差分得到[10,45,-10,-20,0]
        // 然后求前缀和得到[10,55,45,25,25]正好是答案
        // O(n + m)
        // AC 24ms 3.9mb
        let mut answer = vec![0; n as usize];
        for booking in bookings {
            answer[booking[0] as usize - 1] += booking[2];
            if booking[1] < n {
                answer[booking[1] as usize] -= booking[2];
            }
        }
        for i in 1..n as usize {
            answer[i] += answer[i - 1];
        }
        answer
    }
}