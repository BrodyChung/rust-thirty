
// if a fn needs a mut parameter, use &mut, the definition and call-place should be &mut
// 如果函数需要一个mut参数，则使用&mut，定义和调用地方应该是&mut
fn modify_string(s: &mut String) {
    s.push_str(", and modified");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_mut_fn() {
        let mut my_string = String::from("Original String");
        modify_string(&mut my_string);
        assert_eq!(my_string, "Original String, and modified");
    }
}