// ref: https://www.codewars.com/kata/54b724efac3d5402db00065e/train/rust
// 
mod preloaded;
use preloaded::MORSE_CODE;

// ref: https://dev.to/dmarcr1997/rust-fundamentals-with-a-morse-code-decoder-4hm8
// ! to many to_string()
// parse() to turbofish parse::<u32>(), 
// 原实现写了太多的to_string()，也就是从&str变成String，需要消耗资源
// parse() 函数可以根据左边的类型来决定右边的输出，但最好采用turbofish这样不至于引起困扰
pub fn decode_morse(encoded: &str) -> String {
    let refined: Vec<&str> = encoded
    .split(" ")
    //.map(|s| s)
    .collect();

    let mut decoded_string: Vec<&str> = vec![];
    let mut space_count = 0;
    for code in refined {
        if code == "" {
            if space_count < 1 {
                space_count += 1;
                decoded_string.push(" ");
            } else {
                space_count = 0;
            }
        } else {
            for (k, v) in MORSE_CODE.iter() {
                if k.to_string() == code {
                    decoded_string.push(v);
                    break;
                }
            }
        }
    }
    return decoded_string.join("").trim().to_string();
}

// ref: https://doc.rust-lang.org/rust-by-example/testing/unit_testing.html
// only run this test: cargo test test_hey_jude
// 在莫尔斯码中，每个字母或数字的编码是由点（.）和横线（-）组成，
// 它们之间用空格分隔。字母之间通常用一个空格表示，而单词之间通常用三个空格表示
// 这只是简化后的莫尔斯码，
/*
Y: -.--
0: -----
O: ---
M: --
T: -
5: .....
H: ....
S: ...
I: ..
E: .
SOS: ... --- ...
*/
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hey_jude() {
        assert_eq!(decode_morse(".... . -.--   .--- ..- -.. ."), "HEY JUDE");
        assert_eq!(decode_morse("... --- ..."), "SOS");
    }
}