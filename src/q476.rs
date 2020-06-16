mod q476 {
    pub fn find_complement(num: i32) -> i32 {
        let lz = num.leading_zeros();
        !num << lz >> lz
    }
}