struct Solution;

impl Solution {
    pub fn build_array(target: Vec<i32>, n: i32) -> Vec<String> {
        // 方法1
        // target.last() + 1长度的一个数组arr，按照target[i]作为下标设置值为1
        // 构建返回数组res，从下标1开始迭代arr
        //    如果arr[i] = 1并且i <= n， 放入"Push"
        //    如果arr[i] = 0或者i > n，放入"Push"和"Pop"
    }
}