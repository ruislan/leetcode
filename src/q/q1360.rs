use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn days_between_dates(date1: String, date2: String) -> i32 {
        // 方法1
        // 得到abs(year1 - year2) * 365，注意中间如果有润年，则要加366
        // 得到1|3|5|7|8|10|12为31,4|6|9|11为30天，如果year是闰年则，2为29,否则为28
        // 闰年是除以4和100余数为0的即是闰年
        // (year2 - year1) * 365 + m1 + d1 - m2 - d2
        // Passed 0ms 2.1mb
        // let ymd1: Vec<i32> = date1.split("-").map(|s| match s.parse::<i32>() {
        //     Ok(n) => n,
        //     Err(e) => 0,
        // }).collect();
        //
        // let ymd2: Vec<i32> = date2.split("-").map(|s| match s.parse::<i32>() {
        //     Ok(n) => n,
        //     Err(e) => 0,
        // }).collect();
        //
        // let (mut days, mut min, mut max) = (0, vec![], vec![]);
        // if ymd1[0] > ymd2[0] {
        //     min = ymd2;
        //     max = ymd1;
        // } else if ymd1[0] < ymd2[0] {
        //     min = ymd1;
        //     max = ymd2;
        // } else {
        //     if ymd1[1] > ymd2[1] {
        //         min = ymd2;
        //         max = ymd1;
        //     } else if ymd1[1] < ymd2[1] {
        //         min = ymd1;
        //         max = ymd2;
        //     } else {
        //         if ymd1[2] > ymd2[2] {
        //             min = ymd2;
        //             max = ymd1;
        //         } else if ymd1[2] < ymd2[2] {
        //             min = ymd1;
        //             max = ymd2;
        //         } else {
        //             min = ymd1;
        //             max = ymd2;
        //         }
        //     }
        // }
        //
        // (min[0]..max[0]).for_each(|year| {
        //     if (year % 4 == 0 && year % 100 != 0) || year % 400 == 0 { days += 366; } else { days += 365; }
        // });
        //
        // (1..min[1]).for_each(|month| {
        //     match month {
        //         1 | 3 | 5 | 7 | 8 | 10 | 12 => days -= 31,
        //         4 | 6 | 9 | 11 => days -= 30,
        //         _ => days -= if (min[0] % 4 == 0 && min[0] % 100 != 0) || min[0] % 400 == 0 { 29 } else { 28 }
        //     }
        // });
        // days -= min[2];
        //
        // (1..max[1]).for_each(|month| {
        //     match month {
        //         1 | 3 | 5 | 7 | 8 | 10 | 12 => days += 31,
        //         4 | 6 | 9 | 11 => days += 30,
        //         _ => days += if (max[0] % 4 == 0 && max[0] % 100 != 0) || max[0] % 400 == 0 { 29 } else { 28 }
        //     }
        // });
        // days += max[2];
        //
        // days

        // 方法2
        // 方法1需要知道谁大谁小，增加了一些代码量，我们知道输入的日期是从1971年开始，所以，我们只需要求出
        // ymd1 - 1971-01-01 的days1和 ymd2 - 1971-01-01的days2，两个相减的绝对值即可，不需要知道谁大谁小
        // Passed 0ms 2.1mb
        let ymd1: Vec<i32> = date1.split('-').map(|s| s.parse().unwrap_or(0)).collect();
        let ymd2: Vec<i32> = date2.split('-').map(|s| s.parse().unwrap_or(0)).collect();
        (Self::days_from_1971(ymd1) - Self::days_from_1971(ymd2)).abs()
    }

    fn days_from_1971(ymd: Vec<i32>) -> i32 {
        (1971..=ymd[0]).map(|year| {
            let is_leap = (year % 4 == 0 && year % 100 != 0) || year % 400 == 0;
            if year != ymd[0] {
                if is_leap { 366 } else { 365 }
            } else {
                (1..ymd[1]).map(|month|
                    match month {
                        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
                        4 | 6 | 9 | 11 => 30,
                        _ => if is_leap { 29 } else { 28 }
                    }
                ).sum::<i32>() + ymd[2]
            }
        }).sum()
    }
}

#[test]
fn test_q1360() {
    assert_eq!(Solution::days_between_dates("1971-06-29".to_string(), "2010-09-23".to_string()), 14331);
    assert_eq!(Solution::days_between_dates("2019-06-29".to_string(), "2019-06-30".to_string()), 1);
    assert_eq!(Solution::days_between_dates("2019-06-30".to_string(), "2019-06-29".to_string()), 1);
    assert_eq!(Solution::days_between_dates("2019-06-29".to_string(), "2019-06-29".to_string()), 0);
    assert_eq!(Solution::days_between_dates("2020-01-15".to_string(), "2019-12-31".to_string()), 15);
    assert_eq!(Solution::days_between_dates("2000-03-30".to_string(), "2019-03-30".to_string()), 6939);
    assert_eq!(Solution::days_between_dates("1971-02-01".to_string(), "2020-06-28".to_string()), 18045);
}