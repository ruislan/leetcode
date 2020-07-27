mod q1185 {
    pub fn day_of_the_week(day: i32, month: i32, year: i32) -> String {
        let month_strs = vec!["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"];

        let mut days = day;
        for m in 1..month {
            match m {
                1 | 3 | 5 | 7 | 8 | 10 | 12 => days += 31,
                2 => days += if (year % 4 == 0 && year % 100 != 0) || year % 400 == 0 { 29 } else { 28 },
                _ => days += 30,
            }
        }
        for y in 1..year {
            days += if (y % 4 == 0 && y % 100 != 0) || y % 400 == 0 { 366 } else { 365 };
        }
        month_strs[(days % 7) as usize].to_string()

        // 公式法， [Y-1] + [(Y-1)/4] - [(Y-1)/100] + [(Y-1)/400] + D ，D是一年中的第几天
        // Passed 0ms 2.0mb
        //
        // let mut days = day;
        // for m in 1..month {
        //     match m {
        //         1 | 3 | 5 | 7 | 8 | 10 | 12 => days += 31,
        //         2 => days += if (year % 4 == 0 && year % 100 != 0) || year % 400 == 0 { 29 } else { 28 },
        //         _ => days += 30,
        //     }
        // }
        // let dw = year - 1 + (year - 1) / 4 - (year - 1) / 100 + (year - 1) / 400 + days;
        // month_strs[(dw % 7) as usize].to_string()

        // 1971年1月1日是星期5，所以这个方法要+4
        // Passed 0ms 2.0mb

        // let mut days = day;
        // for m in 1..month {
        //     match m {
        //         1 | 3 | 5 | 7 | 8 | 10 | 12 => days += 31,
        //         2 => days += if (year % 4 == 0 && year % 100 != 0) || year % 400 == 0 { 29 } else { 28 },
        //         _ => days += 30,
        //     }
        // }
        //
        // for y in 1971..year {
        //     days += if (y % 4 == 0 && y % 100 != 0) || y % 400 == 0 { 366 } else { 365 };
        // }
        //
        // days += 4;
        //
        // month_strs[(days % 7) as usize].to_string()
    }
}
