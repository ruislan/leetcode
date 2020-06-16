mod q877 {
    pub fn stone_game(piles: Vec<i32>) -> bool {
        let (mut left, mut right) = (0, piles.len() - 1);
        let (mut alex, mut lee) = (0, 0);

        while left + 1 < right - 1 {
            let (nL, nLL, nRR, nR) = (piles[left], piles[left + 1], piles[right - 1], piles[right]);
            let (mut tmpA0, mut tmpA1) = (nL, nR);

            tmpA0 += if nLL > nR { std::cmp::max(nRR, nR) } else { std::cmp::max(nLL, nRR) };
            tmpA1 += if nL > nRR { std::cmp::max(nLL, nRR) } else { std::cmp::max(nL, nLL) };

            if tmpA0 > tmpA1 {
                alex += nL;
                lee += nR;
            } else {
                alex += nR;
                lee += nL;
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