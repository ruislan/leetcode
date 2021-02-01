use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn count_good_rectangles(rectangles: Vec<Vec<i32>>) -> i32 {
        // 方法1
        // 边长最短的那个作为正方向的边
        // 如果这个边比最大的那个边还要大，那么重新计数
        // 如果和最大的那个边一样，那么计数+1
        // AC 4ms 2.1mb
        let mut max = 0;
        let mut answer = 0;
        for rect in rectangles {
            let k = rect[0].min(rect[1]);
            if k == max {
                answer += 1;
            } else if k > max {
                max = k;
                answer = 1;
            }
        }
        answer
    }
}