use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn number_to_words(num: i32) -> String {
        // 方法1
        // 3位算一组，然后每组里面计算百十个，然后外面得到十亿，百万，千
        // 每组里面又分为了20以内（因为英文20以内是单个单词），20-100，以及100以上
        // AC 0ms 2mb 601/601
        let singles = ["", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine"];
        let teens = ["Ten", "Eleven", "Twelve", "Thirteen", "Fourteen", "Fifteen", "Sixteen", "Seventeen", "Eighteen", "Nineteen"];
        let tens = ["", "Ten", "Twenty", "Thirty", "Forty", "Fifty", "Sixty", "Seventy", "Eighty", "Ninety"];
        let thousands = ["", "Thousand", "Million", "Billion"];
        let to_english = |mut num: usize| -> String {
            let mut curr = String::new();
            let mut hundred = num / 100;
            num %= 100;
            if hundred != 0 {
                curr.push_str(singles[hundred]);
                curr.push_str(" Hundred ");
            }
            let ten = num / 10;
            if ten >= 2 {
                curr.push_str(tens[ten]);
                curr.push(' ');
                num %= 10;
            }
            if num > 0 && num < 10 {
                curr.push_str(singles[num]);
                curr.push(' ');
            } else if num >= 10 {
                curr.push_str(teens[num - 10]);
                curr.push(' ');
            }
            return curr;
        };

        if num == 0 { return String::from("Zero"); }
        let mut ans = String::new();
        let mut i = 3;
        let mut unit = 1000000000;
        let mut num = num;
        while i < 4 {
            let mut cur_num = num / unit;
            if cur_num != 0 {
                num -= cur_num * unit;
                ans.push_str(&to_english(cur_num as usize));
                ans.push_str(thousands[i]);
                ans.push(' ');
            }
            unit /= 1000;
            i -= 1;
        }
        ans.trim().to_string()
    }
}
