use crate::q::Solution;

impl Solution {
    pub fn sum_zero(n: i32) -> Vec<i32> {
        // 方法1：数组内部相加等于0最好的方式就是这个数和它的负数，如(1,-1),(2,-2)
        // 我们罗列n=1,2,3,4,5,6
        // n = 1, [0]
        // n = 2, [-1,1]
        // n = 3, [-1, 0, 1]
        // n = 4, [-2,-1, 1, 2]
        // n = 5, [-2, -1, 0, 1, 2]
        // n = 6, [-3, -2, -1, 1, 2, 3]
        // 可以看出，n 为(2, 3)时，只相差0，为(4, 5)时， 只相差0
        // Passed 0ms 2mb
        // let mut res = Vec::new();
        // for x in 1..=(n / 2) {
        //     res.push(x);
        //     res.push(-x);
        // }
        // if n & 1 != 0 { res.push(0); }
        // res

        // 只是方法1的不同写法
        let mut res = Vec::new();
        (1..(n / 2)).for_each(|x| {
            res.push(x);
            res.push(-x);
        });
        if n & 1 != 0 { res.push(0); }
        res

        // 方法2（看的别人的题解）：1 - n即是n 的另外半边少1，然后我们只需要每隔1个取值即可
        // 例如 n = 3， -2, -1, 0, 1, 2, 3， 取值 -2, 0, 2
        // 例如 n = 4, -3, -2, -1, 0, 1, 2, 3, 4， 取值 -3, -1, 1, 3
        // 对比方法1，这个方法简洁美观，但是迭代数据多出了一半（性能影响极低，都是O(n)），装逼非常有用
        // ((1 - n)..n).step_by(2).collect()
    }
}

#[test]
fn test_q1304() {
    let n = 10;
    (0..n).for_each(|n| {
        assert_eq!(0, Solution::sum_zero(n).iter().sum());
    });
}