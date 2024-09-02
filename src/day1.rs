// ref: https://www.codewars.com/kata/5259b20d6021e9e14c0010d4/train/rust
// split vs split_whitespace
// vec & collect()
pub fn reverse_words(str: &str) -> String {
    // let words: Vec<String> = str.split_whitespace()
    let words: Vec<String> = str.split(" ")
    .map(|word| word.chars().rev().collect())
    .collect();

    words.join(" ")
}

pub fn run() {
    let words: &str = "Hello world";
    let ret: String = reverse_words(words);
    println!("{words}");
    println!("{ret}");

    let mut ret2: String = reverse_words(words);
    ret2.push_str("!");
    println!("{ret2}");

    //
    let ret3 = first_word(words);
    println!("{ret3}");
}

// 输出第一个单词 - 没有所有权 - 全是
fn first_word(s: &str) -> &str {
  let bytes = s.as_bytes();
  for (i, &item) in bytes.iter().enumerate() {
      if item == b' ' {
          return &s[0..i];
      }
  }
  &s[..]
}

#[test]
fn sample_test() {
  assert_eq!(reverse_words("The quick brown fox jumps over the lazy dog."), "ehT kciuq nworb xof spmuj revo eht yzal .god");
  assert_eq!(reverse_words("apple"), "elppa");
  assert_eq!(reverse_words("a b c d"),"a b c d");
  assert_eq!(reverse_words("double  spaced  words"), "elbuod  decaps  sdrow");
}