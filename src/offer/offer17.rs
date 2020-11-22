use crate::offer::Solution;

#[allow(unused)]
impl Solution {
    pub fn print_numbers(n: i32) -> Vec<i32> {
        // 方法1
        // n代表位数，用公式10.pow(n) - 1求到最大的n位数记为max
        // 例如n = 4, 10.pow(5)= 1万，减去1为9999
        // 然后迭代1..=max，并map每个数字，然后收集为vec即可
        // 这里返回的是i32的数组，所以结果是不会超出i32的范围，所以不用考虑大数
        (1..10_i32.pow(n as u32)).collect()
    }

    pub fn print_big_numbers(n: i32) -> Vec<String> {
        // 假设我们返回Vec<String>，String是大数据，大数字打印
        // 方法1
        // 利用库函数biguint来处理
        // 处理4位和5位数字的时间是在100ms左右，6位在800ms左右
        // (1..10_u128.pow(n as u32)).map(|x| x.to_string()).collect()

        // 方法2
        // 如果无限大数，那么就需要用vec来存储当前数字的每个位置上的数字
        // 每打印一个数字，就需要从个位遍历vec，检查进位，然后将数组转换成字符串添加到结果中
        // 一直到所有的stack都pop完之后，得到一个字符串即为当前数字
        // 这个算法效率不行，O(n*n)级别，就是这个级别都还要优化，现在是数字转换成字符串太慢了
        // 数字转换成字符串5位数会在517ms左右，现在替换成了48+x转换成ascii的char放入，效率提升了5倍，现在是100ms左右
        // 6位数会在6s左右，替换成char之后是800ms左右
        // 这个结果和方法1的效率是一样的，说明主要的效率在于转换成字符串，如果不需要转换成字符串，那么下列算法应该算是达到库函数级别的
        let mut num = vec![];
        let mut res = Vec::new();
        while num.len() < (n as usize + 1) {
            let mut carry = 1;
            for i in 0..num.len() {
                num[i] += carry;
                if num[i] == 10 {
                    carry = 1;
                    num[i] = 0;
                } else {
                    carry = 0;
                    break;
                }
            }
            if carry == 1 { num.push(1); }
            res.push(num.iter().rev().map(|&x| (48_u8 + x) as char).collect());
        }
        res.pop();
        res
    }
}

#[test]
fn test() {
    assert_eq!(Solution::print_numbers(0), vec![]);
    assert_eq!(Solution::print_numbers(1), vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);

    // assert_eq!(Solution::print_big_numbers(0), Vec::new());
    // assert_eq!(Solution::print_big_numbers(1), vec!["1".to_string(), "2".to_string(), "3".to_string(), "4".to_string(), "5".to_string(), "6".to_string(), "7".to_string(), "8".to_string(), "9".to_string()]);
    assert_eq!(Solution::print_big_numbers(4).len(), 9999);
    assert_eq!(Solution::print_big_numbers(5).len(), 99999);
    // assert_eq!(Solution::print_big_numbers(6).len(), 999999);
    // assert_eq!(Solution::print_big_numbers(7).len(), 9999999);
}