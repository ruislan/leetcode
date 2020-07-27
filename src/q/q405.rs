mod q405 {
    pub fn to_hex(num: i32) -> String {
        // 方法1
        // format!("{:x}", num)

        
        // 方法2
        let chars = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f'];
        let mut res = String::new();
        let mut num = num as u32;
        while num > 0 {
            res.insert(0, chars[(num % 16) as usize]);
            num /= 16;
        }
        if res.is_empty() { String::from("0") } else { res }
    }
}