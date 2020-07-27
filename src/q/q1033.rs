mod q1033 {
    pub fn num_moves_stones(a: i32, b: i32, c: i32) -> Vec<i32> {
        let mut v = vec![a, b, c];
        v.sort();
        let (x, y, z) = (v[0], v[1], v[2]);
        let (mut min, mut max) = (0, 0);
        max += z - x - 2;
        if z - x == 2 { min = 0; } else if y - x <= 2 || z - y <= 2 { min = 1; } else { min = 2; }
        vec![min, max]
    }
}