use crate::q::Solution;

impl Solution {
    pub fn construct_rectangle(area: i32) -> Vec<i32> {
        // 方法1
        // 将area开方，这样就得到一个中心值，
        // 然后w和l则在这个中心值得基础上扩展和减少
        // 设 product = w * l
        // 假设product > area，则说明需要减少w或l的某个值，因为条件要求w <= l，则优先减少w
        // 假设product < area，则说明需要增加w或l的某个值，因为条件要求w <= l，则优先增加l
        // 假设product == area，则说明我们找到了这个值，直接返回
        // 因为area是正整数，那么说明其必然能被1或者自己整除，那么必然有结果
        // 也即是必然会出现product == area
        // Passed 4ms 2mb
        // let mut w = (area as f32).sqrt() as i32;
        // let mut l = w.clone();
        // loop {
        //     let product = w * l;
        //     if product < area { l += 1 } else if product > area { w -= 1; } else { return vec![l, w]; }
        // }

        // 方法2
        // 其实我们不用两边都调整，只需要调整w即可
        // Passed 0ms 2.1mb
        let mut w = (area as f32).sqrt() as i32;
        while area % w != 0 { w -= 1 }
        vec![area / w, w]
    }
}

#[test]
fn test() {
    assert_eq!(Solution::construct_rectangle(1), vec![1, 1]);
    assert_eq!(Solution::construct_rectangle(2), vec![2, 1]);
    assert_eq!(Solution::construct_rectangle(3), vec![3, 1]);
    assert_eq!(Solution::construct_rectangle(4), vec![2, 2]);
    assert_eq!(Solution::construct_rectangle(5), vec![5, 1]);
    assert_eq!(Solution::construct_rectangle(6), vec![3, 2]);
    assert_eq!(Solution::construct_rectangle(7), vec![7, 1]);
    assert_eq!(Solution::construct_rectangle(8), vec![4, 2]);
    assert_eq!(Solution::construct_rectangle(9), vec![3, 3]);
    assert_eq!(Solution::construct_rectangle(10), vec![5, 2]);
    assert_eq!(Solution::construct_rectangle(11), vec![11, 1]);
    assert_eq!(Solution::construct_rectangle(12), vec![4, 3]);
    assert_eq!(Solution::construct_rectangle(13), vec![13, 1]);
    assert_eq!(Solution::construct_rectangle(14), vec![7, 2]);
    assert_eq!(Solution::construct_rectangle(15), vec![5, 3]);
    assert_eq!(Solution::construct_rectangle(10000000), vec![3200, 3125]);
}