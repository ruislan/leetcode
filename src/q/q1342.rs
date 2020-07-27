mod q1342 {
    #[test]
    fn test_q1342() {
        assert_eq!(number_of_steps(0), 0);
        assert_eq!(number_of_steps(8), 4);
        assert_eq!(number_of_steps(123), 12);
        assert_eq!(number_of_steps(14), 6);
    }

    pub fn number_of_steps(num: i32) -> i32 {
        // 方法1
        // if num % 2 == 0, num /= 2，次数加1
        // else , (num - 1) /= 2， 次数加2
        // 统计次数
        // Passed 0ms 2mb
        // let (mut count, mut num) = (0, num);
        // while num > 0 {
        //     if num % 2 == 0 || num == 1 {
        //         num = num >> 1;
        //         count += 1;
        //     } else {
        //         num = (num - 1) >> 1;
        //         count += 2;
        //     }
        // }
        // count

        // 方法2
        // 除以2的次数，实际上为二进制的实际位数m，同时有n个1，则需要减去1的次数，因为最后一位不需要除以2，所以公式为m + n - 1
        // 注意0的话，会成为-1，所以直接抹平为0
        // 例如4是100，则是3 + 1 - 1 = 3
        // 例如5是101，则是3 + 2 - 1 = 4
        // 例如8是1000，则是 4 + 1 - 1 = 4
        (2 * num.count_ones() + num.count_zeros() - num.leading_zeros()).saturating_sub(1) as i32
    }
}