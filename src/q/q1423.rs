use crate::q::Solution;

#[allow(unused)]
impl Solution {
    // 方法名与1422一样，修改为max_score_card_points
    pub fn max_score_card_points(card_points: Vec<i32>, k: i32) -> i32 {
        // 方法1
        // 把问题转换一下就变成了求n - k个连续子数组的和最小的
        // 这样左右两边拿走的k张牌自然就是最大的
        // AC 8ms 3~3.2mb
        let n = card_points.len();
        let k = n - k as usize;
        let mut sum = 0;
        let mut window_sum = 0;
        for i in 0..n { sum += card_points[i]; }
        for i in 0..k { window_sum += card_points[i]; }

        let mut answer = window_sum;
        for i in k..n {
            window_sum += card_points[i];
            window_sum -= card_points[i - k];
            answer = answer.min(window_sum);
        }
        sum - answer
    }
}