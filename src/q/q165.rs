use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        // 方法1
        // 分割字符串，转换成数字数组，将短数组后面补0成一样长度，再比较即可
        // AC 0ms 2.1mb
        let version1: Vec<i32> = version1.split(".").map(|x| {
            x.parse::<i32>().unwrap()
        }).collect();
        let version2: Vec<i32> = version2.split(".").map(|x| {
            x.parse::<i32>().unwrap()
        }).collect();

        let n = version1.len().max(version2.len());
        let mut v1 = vec![0; n];
        let mut v2 = vec![0; n];
        version1.iter().enumerate().for_each(|(i, &x)| v1[i] = x);
        version2.iter().enumerate().for_each(|(i, &x)| v2[i] = x);
        for i in 0..n {
            if v1[i] > v2[i] { return 1; } else if v1[i] < v2[i] { return -1; }
        }
        0

        // 方法2
        // 虽然题里面没有要求不用额外空间，
        // 但是如果不用额外空间的话就直接双指针遍历
        // 设置两个数字a,b分别表示当前修订号，也就是"."的前面或者后面，或者两个"."中间的数字，
        // 遇到"."就解析前面的字符串成数字，然后两个数字比较
        // 相等就继续，大或者小就直接返回
        // 这样额外的空间就只有几个临时参数了。
        // 今天时间不够了，后面有每日一题再来补吧。
    }
}