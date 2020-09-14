use crate::q::Solution;

impl Solution {
    pub fn is_long_pressed_name(name: String, typed: String) -> bool {
        let mut n = Vec::new();
        let mut ch = None;
        let mut count = 0;
        for chn in name.chars() {
            if None == ch {
                ch = Some(chn);
                count = 1;
            } else if Some(chn) == ch {
                count += 1;
            } else {
                n.push((ch.unwrap(), count));
                ch = Some(chn);
                count = 1;
            }
        }
        if count > 0 { n.push((ch.unwrap(), count)); }

        let mut t = Vec::new();
        ch = None;
        count = 0;
        for cht in typed.chars() {
            if None == ch {
                ch = Some(cht);
                count = 1;
            } else if Some(cht) == ch {
                count += 1;
            } else {
                t.push((ch.unwrap(), count));
                ch = Some(cht);
                count = 1;
            }
        }
        if count > 0 { t.push((ch.unwrap(), count)); }

        if t.len() != n.len() { return false; }
        for i in 0..t.len() {
            if t[i].0 != n[i].0 { return false; }
            if t[i].1 < n[i].1 { return false; }
        }
        return true;
    }
}