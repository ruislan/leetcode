use crate::interview::Solution;

#[allow(unused)]
impl Solution {
    pub fn merge(a: &mut Vec<i32>, m: i32, b: &mut Vec<i32>, n: i32) {
        // 方法1
        // 从m位置开始直接填充b即可，然后对a进行排序
        // Rust排序很快，虽然O(nlogn)
        // Passed 0ms 2.1mb
        // let (m, n) = (m as usize, n as usize);
        // for i in m..m + n {
        //     a[i] = b[i - m];
        // }
        // a.sort()

        // 方法2
        // 因为a,b有序，所以我们比较a[i]和b[j]
        // 创建一个与a相同长度的数组arr
        // 迭代当 i < m 且 j < n：
        //  如果a[i] <= b[j]，则a[i]放入arr,i += 1
        //  如果a[i] > b[j]，则b[j]放入arr，j += 1
        // 完成迭代
        // 如果i小于m，将a[i]的内容接着放入arr
        // 如果j小于n，将a[j]的内容接着放入arr
        // 将arr放入a[i]
        // Passed 0ms 2mb
        // let (mut m, mut n) = (m as usize, n as usize);
        // let (mut j, mut i) = (0, 0);
        // let mut arr = Vec::new();
        // while i < m && j < n {
        //     if a[i] <= b[j] {
        //         arr.push(a[i]);
        //         i += 1;
        //     } else {
        //         arr.push(b[j]);
        //         j += 1;
        //     }
        // }
        // while i < m {
        //     arr.push(a[i]);
        //     i += 1;
        // }
        // while j < n {
        //     arr.push(b[j]);
        //     j += 1;
        // }
        // std::mem::replace(a, arr);

        // 方法3
        // 方法2要多一个数组，如何不用额外的数组？
        // 如果我们将上面的数组按从大的放起如何？
        let (mut i, mut j, mut k) = (m - 1, n - 1, (m + n - 1) as usize);
        while i >= 0 || j >= 0 {
            if i < 0 {
                a[k] = b[j as usize];
                j -= 1;
            } else if j < 0 {
                a[k] = a[i as usize];
                i -= 1;
            } else if a[i as usize] > b[j as usize] {
                a[k] = a[i as usize];
                i -= 1;
            } else {
                a[k] = b[j as usize];
                j -= 1;
            }
            k = k.saturating_sub(1);
        }
    }
}

#[test]
fn test() {
    let mut a = vec![];
    let mut b = vec![];
    Solution::merge(&mut a, 0, &mut b, 0);
    assert_eq!(a, vec![]);

    let mut a = vec![1];
    let mut b = vec![];
    Solution::merge(&mut a, 1, &mut b, 0);
    assert_eq!(a, vec![1]);

    let mut a = vec![0];
    let mut b = vec![1];
    Solution::merge(&mut a, 0, &mut b, 1);
    assert_eq!(a, vec![1]);

    let mut a = vec![4, 5, 6, 0, 0, 0];
    let mut b = vec![1, 2, 3];
    Solution::merge(&mut a, 3, &mut b, 3);
    assert_eq!(a, vec![1, 2, 3, 4, 5, 6]);


    let mut a = vec![1, 2, 3, 0, 0, 0];
    let mut b = vec![2, 5, 6];
    Solution::merge(&mut a, 3, &mut b, 3);
    assert_eq!(a, vec![1, 2, 2, 3, 5, 6]);
}