// 方法1
// 这里并没有限制加密成多短的，所以我们有好几个方案
// 第一种方案，我们通过锁的方式，保证每次掉用的id是唯一增长的
// 这种方案单机是完全没有问题的，如果有多个机器，就可能会出现问题
// 第二个方案就是利用设计好的分布式id生成策略，保证key的唯一性
// 例如UUID就是一个可用方案，sha256也可以是一个方案，
// 还有一些其他的方案，例如用机器的id，加上mac地址，加上时间戳，加上随机数等等方式
// 鉴于这道题的验证应该不存在多线程和多机器，所以我们可以简单的用无锁id自增的方式处理
// 至于解密，我们可以采用hashmap存储tinyurl和origin_url
// AC 0ms 2.1mb
use std::collections::HashMap;

struct Codec {
    cache: HashMap<String, String>,
    id: i32,
}

#[allow(unused)]
impl Codec {
    fn new() -> Self {
        Codec { cache: HashMap::new(), id: 0 }
    }

    fn encode(&mut self, long_url: String) -> String {
        self.id = self.id + 1;
        let tiny_url = format!("http://tinyurl.com/{:?}", self.id);
        self.cache.insert(tiny_url.clone(), long_url);
        tiny_url
    }

    fn decode(&self, short_url: String) -> String {
        if let Some(url) = self.cache.get(&short_url) {
            url.clone()
        } else {
            String::new()
        }
    }
}