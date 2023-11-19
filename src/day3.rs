
// ref: https://docs.rs/shuffle/latest/shuffle/
use rand::seq::SliceRandom; // shuffle()
use rand::thread_rng;

// shuffle: using Fisher-Yates algorithm - O(n)
// irs is another shuffle algorithm
// shuffle这个方法是修改str，而不是返回值因此无法使用chain
// 最后的collect::<String>() 直接从Vec<char>的iter到String - collect的特殊用法
// 一般的collect都是从iter到Vec
pub fn shuffle(str: &str) -> String {
    let mut rng = thread_rng();
    let mut string_to_shuffle: Vec<char> = str.chars().collect();
    string_to_shuffle.shuffle(&mut rng);
    return string_to_shuffle.iter().collect::<String>()
}

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