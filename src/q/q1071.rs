use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        let _str_min = if str1.len() > str2.len() { str2.clone() } else { str1.clone() };
        let _str_max = if str1.len() > str2.len() { str1 } else { str2 };

        if _str_max.len() % _str_min.len() == 0 {
            let mut g = _str_max.len() / _str_min.len();
            let mut t = String::new();
            while g > 0 {
                t.push_str(_str_min.as_str());
                g -= 1;
            }
            if t == _str_max {
                return _str_min.clone();
            }
        }

        for i in (1..=(_str_min.len() / 2)).rev() {
            if _str_min.len() % i == 0 {
                let mut s = _str_min.clone();
                s.truncate(i);
                let (mut g_min, mut g_max) = (_str_min.len() / i, _str_max.len() / i);
                let (mut t_min, mut t_max) = (String::new(), String::new());
                while g_min > 0 {
                    t_min.push_str(s.as_str());
                    g_min -= 1;
                }
                while g_max > 0 {
                    t_max.push_str(s.as_str());
                    g_max -= 1;
                }
                if t_min == _str_min && t_max == _str_max {
                    return s.clone();
                }
            }
        }
        
        String::new()
    }
}