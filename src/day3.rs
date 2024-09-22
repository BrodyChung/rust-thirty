
// ref: https://docs.rs/shuffle/latest/shuffle/
use rand::seq::SliceRandom; // shuffle()
use rand::thread_rng;

// shuffle: using Fisher-Yates algorithm - O(n)
// irs is another shuffle algorithm
// shuffle这个方法是修改vec，而不是返回值因此无法使用chain
// 最后的collect::<String>() 直接从Vec<char>的iter到String - collect的特殊用法
// 一般的collect都是从iter到Vec
// thread_rng 它提供了一个简单的方式来获取随机数，而不需要管理全局状态或担心线程安全问题。
pub fn shuffle(str: &str) -> String {
    let mut rng = thread_rng();
    let mut string_to_shuffle: Vec<char> = str.chars().collect();
    string_to_shuffle.shuffle(&mut rng);
    return string_to_shuffle.iter().collect::<String>()
}

// "cargo test -- --nocapture" 会输出print到标准输出
// "cargo test test_shuffle" 仅仅执行某个测试
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_shuffle() {
        let jude = "Hey Jued";
        assert_ne!(shuffle(jude), jude); // might fail
        assert_eq!(shuffle(jude).chars().count(), jude.chars().count());
    }
}