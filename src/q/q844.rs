mod q_844 {
    #[test]
    fn test_q_844() {
        assert_eq!(
            true,
            backspace_compare("ab#c".to_string(), "ad#c".to_string())
        );
        assert_eq!(
            true,
            backspace_compare("ab##".to_string(), "c#d#".to_string())
        );
        assert_eq!(
            true,
            backspace_compare("a##c".to_string(), "#a#c".to_string())
        );
        assert_eq!(false, backspace_compare("a#c".to_string(), "b".to_string()));
    }

    /**
     * 给定 S 和 T 两个字符串，当它们分别被输入到空白的文本编辑器后，判断二者是否相等，并返回结果。 # 代表退格字符。
     */
    pub fn backspace_compare(s: String, t: String) -> bool {
        let (mut a, mut b) = (String::new(), String::new());
        for ch in s.chars() {
            if ch == '#' {
                a.pop();
            } else {
                a.push(ch);
            }
        }
        for ch in t.chars() {
            if ch == '#' {
                b.pop();
            } else {
                b.push(ch);
            }
        }
        a == b
    }
}
