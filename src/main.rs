// an `extern crate` loading macros must be at the crate root
// 这个声明需要放到crate的根文件里面
#[macro_use]
extern crate lazy_static;

// 需要声明mod才会编译
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

fn main() {
    println!("Hello, world!");

    // day6::guess_number();
    // day8::http_server_v1();
    day9::http_server();
}
