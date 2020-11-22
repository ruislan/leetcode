use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn reformat_date(date: String) -> String {
        // 方法1
        // 用hashmap存储day和month
        // date的格式是Day Month Year，利用split_ascii_whitespace分割开
        // hashmap中查询day和month，然后用format("{}-{}-{}",yyyy,MM,dd)的格式放置结果并返回
        // Passed 4ms 2.1mb
        // let mut ymd_map = std::collections::HashMap::new();
        // ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"].iter().enumerate().for_each(|(i, &month)| { ymd_map.insert(month.to_string(), i + 1); });
        // ["1st", "2nd", "3rd", "4th", "5th", "6th", "7th", "8th", "9th", "10th", "11th", "12th", "13th", "14th", "15th", "16th", "17th", "18th", "19th", "20th", "21st", "22nd", "23rd", "24th", "25th", "26th", "27th", "28th", "29th", "30th", "31st"]
        //     .iter().enumerate().for_each(|(i, &day)| { ymd_map.insert(day.to_string(), i + 1); });
        // (1900..=2100).for_each(|year| { ymd_map.insert(year.to_string(), year); });
        // let ymd: Vec<usize> = date.split_ascii_whitespace().map(|x| ymd_map[x]).collect();
        // format!("{:04}-{:02}-{:02}", ymd[2], ymd[1], ymd[0])

        // 方法2
        // hashmap只存放月份，日期直接去掉后面两位，年直接解析
        // Passed 0ms 2.1mb
        let mut map = std::collections::HashMap::new();
        ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"].iter().enumerate().for_each(|(i, &month)| { map.insert(month, i + 1); });
        let ymd: Vec<&str> = date.split_ascii_whitespace().collect();
        format!("{:04}-{:02}-{:02}",
                ymd[2].parse::<usize>().unwrap_or(0),
                map[ymd[1]],
                ymd[0].chars().take_while(|&ch| ch.is_ascii_digit()).collect::<String>().parse::<usize>().unwrap_or(0))
    }
}

#[test]
fn test() {
    // assert_eq!(Solution::reformat_date("".to_string()), "".to_string());
    assert_eq!(Solution::reformat_date("20th Oct 2052".to_string()), "2052-10-20".to_string());
    assert_eq!(Solution::reformat_date("6th Jun 1933".to_string()), "1933-06-06".to_string());
    assert_eq!(Solution::reformat_date("26th May 1960".to_string()), "1960-05-26".to_string());
}