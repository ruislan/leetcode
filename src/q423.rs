mod q423 {
    #[test]
    fn test_q423() {
        assert_eq!(original_digits("".to_string()), "".to_string());
        assert_eq!(original_digits("zeroonetwothreefourfivesixseveneightnine".to_string()), "0123456789".to_string());
        assert_eq!(original_digits("owoztneoer".to_string()), "012".to_string());
        assert_eq!(original_digits("owoztneoerthreethreeeighteight".to_string()), "0123388".to_string());
        assert_eq!(original_digits("fviefuro".to_string()), "45".to_string());
    }

    pub fn original_digits(s: String) -> String {
        // 方法1
        // 创建一个[a-z]的桶bag，将s的每个字符都计数在桶里
        // zero  z:1
        // two   w:1
        // six   x:1
        // eight g:1
        // four  u:1
        // three no "eight", has 'h':1
        // five  no 'four', has 'f':1
        // seven no 'six', has 's':1
        // one  no "zero, two, four', has 'o':1
        // nine no "six, eight, five", has 'i':1
        // Passed 0ms 2.2mb
        let mut bag = vec![0; 26];
        s.bytes().for_each(|ch| { bag[ch as usize - 97] += 1; });

        let mut res = vec![0; 10];
        res[0] = bag[b'z' as usize - 97];
        res[2] = bag[b'w' as usize - 97];
        res[6] = bag[b'x' as usize - 97];
        res[8] = bag[b'g' as usize - 97];
        res[4] = bag[b'u' as usize - 97];
        res[3] = bag[b'h' as usize - 97] - res[8];
        res[5] = bag[b'f' as usize - 97] - res[4];
        res[7] = bag[b's' as usize - 97] - res[6];
        res[1] = bag[b'o' as usize - 97] - res[0] - res[2] - res[4];
        res[9] = bag[b'i' as usize - 97] - res[6] - res[8] - res[5];

        res.iter().enumerate()
            .filter(|(_, &num)| num > 0)
            .map(|(i, &num)| vec![(i as u8 + 48) as char; num])
            .flatten()
            .collect()
    }
}