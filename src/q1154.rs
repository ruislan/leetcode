mod q1154 {
    /// Q1154 Passed 0ms 2.0mb
    pub fn day_of_year(date: String) -> i32 {
        let ymd = date.split("-").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();

        let mut days = ymd[2];
        for i in 1..ymd[1] {
            match i {
                1 | 3 | 5 | 7 | 8 | 10 | 12 => days += 31,
                2 => days += if ymd[0] % 400 == 0 || (ymd[0] % 4 == 0 && ymd[0] % 100 != 0) { 29 } else { 28 },
                _ => days += 30,
            }
        }

        days
    }
}