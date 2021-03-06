use crate::lcp::Solution;

#[allow(unused)]
impl Solution {
    // 此题没有Rust和Kotlin的作答窗口，我们用Java来处理
    fn fraction(cont: Vec<i32>) -> Vec<i32> {
        // 方法1
        // 本体提供了公式，a0 + 1 / (a1 + 1 / (a2 + ..))
        // 这个公式一看就知道要递归了，那么说起递归就需要基线条件和缩小范围
        // 我们看基线条件就是如果只有一个数，那么就直接是a0返回即可
        // 如果是多个数缩小范围就变成了 arr[0] + 1 / fn(arr[1..n])
        // 因为返回的数据是与这个结果等值的分母和分子，且分母分子最大公约数还必须是1，那么问题就来了
        // 如果采用上述算法，会有精度差，输入的数据越多，精度差越大，后面基本无法得到正确的结果
        // 我们需要调整一下算法，要想精度不缺失，可以直接把分子分母带着走就好了
        // 但是a0怎么办？其实a0可以变成 a0 / 1 ，这样就有分母和分子了
        // 那么重新来看这个递归的基线条件
        // 如果只有一个数，就返回 a0 / 1，我们用数组分别放分母分子就是vec![分母,分子]
        // 缩小范围，如果是多个，根据公式的特性，我们每次都要翻转一下分子分母
        // 先翻转vec![分子,分母],然后返回vec![分母*ak + 分子, 分母]
        // 举个例子：[3,2,0,2]
        // 直接到递归的最里面，返回vec![2, 1]
        // 翻转得出vec![1,2]，返回vec![2*0 + 1, 2]
        // 翻转得到vec![2,1]，返回vec![1 * 2 + 2, 1]
        // 翻转得到vec![1,4]，返回vec![4 * 3 + 1, 4]
        // 结果为vec![13,4]，正好分子，分母的最大公约数是1

        // 方法1的递归解法
        // Java Passed 0ms 37.6mb
        // fn fraction(cont: &Vec<i32>, ptr: usize) -> Vec<i32> {
        //     if ptr == cont.len() {
        //         vec![1, 0]
        //     } else {
        //         let mut ret = fraction(cont, ptr + 1);
        //         ret.swap(0, 1);
        //         ret[0] = ret[1] * cont[ptr] + ret[0];
        //         ret
        //     }
        // }
        // fraction(&cont, 0)

        // 方法1的循环解法1
        // Rust only
        // let mut cont = cont;
        // let mut ret = vec![1, 0];
        // while !cont.is_empty() {
        //     let n = cont.pop().unwrap();
        //     ret.swap(0, 1);
        //     ret[0] = ret[1] * n + ret[0];
        // }
        // ret

        // 方法1的循环解法2
        // Java Passed 0ms 37.6mb
        let mut ptr = cont.len() as i32 - 1;
        let mut ret = vec![1, 0];
        while ptr >= 0 {
            ret.swap(0, 1);
            ret[0] = ret[1] * cont[ptr as usize] + ret[0];
            ptr -= 1;
        }
        ret
    }
}

#[test]
fn test() {
    assert_eq!(Solution::fraction(vec![1]), vec![1, 1]);
    assert_eq!(Solution::fraction(vec![3, 2, 0, 2]), vec![13, 4]);
    assert_eq!(Solution::fraction(vec![0, 0, 3]), vec![3, 1]);
}