use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn stone_game(piles: Vec<i32>) -> bool {
        let (mut left, mut right) = (0, piles.len() - 1);
        let (mut alex, mut lee) = (0, 0);

        while left + 1 < right - 1 {
            let (n_l, n_ll, n_rr, n_r) = (piles[left], piles[left + 1], piles[right - 1], piles[right]);
            let (mut tmp_a0, mut tmp_a1) = (n_l, n_r);

            tmp_a0 += if n_ll > n_r { std::cmp::max(n_rr, n_r) } else { std::cmp::max(n_ll, n_rr) };
            tmp_a1 += if n_l > n_rr { std::cmp::max(n_ll, n_rr) } else { std::cmp::max(n_l, n_ll) };

            if tmp_a0 > tmp_a1 {
                alex += n_l;
                lee += n_r;
            } else {
                alex += n_r;
                lee += n_l;
            }

            left += 1;
            right -= 1;
        }

        if piles[left] > piles[right] {
            alex += piles[left];
            lee += piles[right];
        } else {
            alex += piles[right];
            lee += piles[left];
        }

        alex > lee
    }
}