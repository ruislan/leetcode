use crate::q::Solution;

impl Solution {
    pub fn find_kth_positive(arr: Vec<i32>, k: i32) -> i32 {
        // 方法1
        // 设定前一个数字为0，
        // 取出数组当前数字，前去前一个数字得到差值，
        // 差值如果小于k，k减去差值，前一个数字等于当前数值，重复上述
        // 差值如果大于k, 前一个数字加上K即是结果
        // 差值如果等于k，返回当前值减去1
        // Passed 0ms 2.1mb
        // let (mut prev, mut k) = (0, k);
        // for n in arr.into_iter() {
        //     let gap = n - prev - 1;
        //     if gap == k {
        //         return n - 1;
        //     } else if gap > k {
        //         return prev + k;
        //     } else {
        //         k -= gap;
        //         prev = n;
        //     }
        // }
        // if k > 0 { prev + k } else { 0 }

        // 方法2
        // 方法1最差的情况就是要迭代整个数组
        // 因为严格升序，我们可以采用二分查找法
        // 中间点与它当前的索引相减去再减一，就是差了多少个数字，
        // 如果这个gap大于等于k，则说明要找的数字还在左边（即便与gap相等，但是有可能这个数的左边的数也与gap相等，令右边界=当前索引-1，继续循环（注意rust的usize不能小于0）
        // 如果这个gap小于k，则说明要找的数字还在右边，令左边界=当前索引+1，继续循环
        // 循环结束，我们只需要返回左边届+k即可
        // Passed 0ms 2.1mb
        let (mut le, mut ri) = (0_i32, arr.len() as i32 - 1);
        while le <= ri {
            let mid = (le + ri) / 2;
            if arr[mid as usize] - mid - 1 >= k {
                ri = mid - 1;
            } else {
                le = mid + 1;
            }
        }
        le + k
    }
}

#[test]
fn test() {
    assert_eq!(Solution::find_kth_positive(vec![], 0), 0);
    assert_eq!(Solution::find_kth_positive(vec![], 5), 5);
    assert_eq!(Solution::find_kth_positive(vec![7], 2), 2);
    assert_eq!(Solution::find_kth_positive(vec![1, 13, 18], 17), 20);
    assert_eq!(Solution::find_kth_positive(vec![1, 2, 3, 4], 2), 6);
    assert_eq!(Solution::find_kth_positive(vec![2, 3, 4, 6, 11], 2), 5);
    assert_eq!(Solution::find_kth_positive(vec![2, 3, 4, 7, 11], 5), 9);
    assert_eq!(Solution::find_kth_positive(vec![1, 7, 11, 14, 29, 31, 40, 44], 20), 24);
    assert_eq!(Solution::find_kth_positive(vec![1, 7, 11, 14, 29, 31, 40, 44], 10), 13);
    assert_eq!(Solution::find_kth_positive(vec![6, 7, 10, 20, 28, 29, 33, 37, 39, 40, 49, 53, 55, 72, 75, 76, 85, 87, 88, 94, 106, 107, 119, 120, 129, 142, 147, 152, 157, 159, 161, 173, 178, 183, 187, 188, 193, 199, 202, 212, 215, 224, 227, 230, 237, 239, 246, 251, 256, 260, 266, 268, 273, 277, 279, 281, 291, 297, 298, 310, 312, 314, 315, 321, 324, 326, 329, 341, 342, 348, 355, 367, 370, 374, 387, 389, 394, 413, 420, 424, 429, 446, 447, 458, 460, 464, 467, 473, 477, 478, 498, 500, 501, 503, 514, 515, 523, 525, 528, 529, 531, 535, 539, 555, 566, 569, 572, 583, 588, 591, 596, 602, 604, 605, 606, 610, 611, 616, 619, 622, 623, 631, 632, 640, 642, 645, 647, 661, 680, 684, 685, 687, 694, 696, 698, 714, 717, 720, 726, 734, 738, 742, 745, 753, 770, 775, 780, 781, 783, 787, 788, 798, 806, 821, 835, 852, 865, 873, 888, 897, 926, 932, 935, 939, 945, 956, 966, 967, 969, 973, 979, 980, 986, 992, 995, 997], 96), 118);
}