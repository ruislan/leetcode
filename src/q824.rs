mod q_824 {
    #[test]
    fn test_q_824() {
        assert_eq!(
            "Imaa peaksmaaa oatGmaaaa atinLmaaaaa".to_string(),
            to_goat_latin("I speak Goat Latin".to_string())
        );
        assert_eq!("imaa".to_string(), to_goat_latin("i".to_string()));
        assert_eq!("iamaa".to_string(), to_goat_latin("ia".to_string()));
        assert_eq!("hmaa".to_string(), to_goat_latin("h".to_string()));
        assert_eq!("mhmaa".to_string(), to_goat_latin("hm".to_string()));
        assert_eq!(
            "heTmaa uickqmaaa rownbmaaaa oxfmaaaaa umpedjmaaaaaa overmaaaaaaa hetmaaaaaaaa azylmaaaaaaaaa ogdmaaaaaaaaaa".to_string(),
            to_goat_latin("The quick brown fox jumped over the lazy dog".to_string())
        );
    }

    /**
    * 如果单词以元音开头（a, e, i, o, u），在单词后添加"ma"。
    * 例如，单词"apple"变为"applema"。

    * 如果单词以辅音字母开头（即非元音字母），移除第一个字符并将它放到末尾，之后再添加"ma"。
    * 例如，单词"goat"变为"oatgma"。

    * 根据单词在句子中的索引，在单词最后添加与索引相同数量的字母'a'，索引从1开始。
    * 例如，在第一个单词后添加"a"，在第二个单词后添加"aa"，以此类推。
    */
    pub fn to_goat_latin(s: String) -> String {
        let mut res = String::new();
        let mut words = s.split_ascii_whitespace().collect::<Vec<&str>>();
        let mut pos = 0;
        let len = words.len();
        for word in words {
            let mut word = word.to_string();
            if let Some(ch) = word.chars().next() {
                match ch {
                    'a' | 'A' | 'e' | 'E' | 'i' | 'I' | 'o' | 'O' | 'u' | 'U' => {}
                    _ => {
                        let f = word.remove(0);
                        word.push(f);
                    }
                }
            }
            word.push_str("ma");
            pos += 1;
            for i in 0..pos {
                word.push('a');
            }
            res.push_str(&word);
            if pos < len {
                res.push(' ');
            }
        }
        res
    }
}