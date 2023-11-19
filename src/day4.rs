// ref: https://dev.to/sammyshear/30-days-of-rust-day-two-25j8
use evalexpr::*;
use std::io;
use std::fs::File;

// There's no function overloading in Rust. 
// rust函数没有重载，只能取不同的名字
pub fn eval_expr() {
    let mut line= String::new();
    println!("Enter your expression: ");
    io::stdin().read_line(&mut line).unwrap();
    print!("{}", &(&line[..]));
    let out = eval(&(&line[..])).unwrap();
    println!(" = {:?}", out);
}

// ref: https://docs.rs/evalexpr/latest/evalexpr/enum.Value.html
// pass: a = 2; a + 4
// fail: a + 2  - 因为a没有定义
// Value是个枚举，也就是支持的类型
// EvalexprResult 和 io::Result是一样的模式
pub fn eval_expr_0(str: &str) -> EvalexprResult<Value> {
    eval(str)
}

// () means Void
// () is a value of the type () and its purpose is to be useless
// pub type std::io::Result<T> = Result<T, Error>
// pub fn read_line(&self, buf: &mut String) -> Result<usize>
// Void类型表示为(),也就是没有输出或者忽略，最后返回- Ok(Void)
// Error是 std::io::Error 是一个struct
// 而 io::Result<T>则是包装了 std::Result<T, E> - 这是一种常见模式
// 当read_line报错的时候，整个函数返回返回 Result.Error，就是最后的问号的作用
fn read() -> io::Result<()> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut buffer)?; // return Error if
    Ok(())
}


fn open() -> Result<(), io::Error> {
    let f = File::open("bar.txt")?;
    Ok(())
}