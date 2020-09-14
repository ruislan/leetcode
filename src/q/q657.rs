use crate::q::Solution;

impl Solution {
    pub fn judge_circle(moves: String) -> bool {
        let mut lr = 0;
        let mut ud = 0;
        for ch in moves.chars() {
            if ch == 'U' { ud += 1; } else if ch == 'D' { ud -= 1; } else if ch == 'L' { lr -= 1; } else if ch == 'R' { lr += 1; }
        }
        lr == 0 && ud == 0
    }
}