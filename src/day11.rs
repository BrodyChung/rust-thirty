// ref: https://github.com/conradludgate/wordle
// clone build & run

/*
 * tui - https://docs.rs/tui/latest/tui/
 * tui是用在termial上显示ui的lib
 * 
 * Serde - https://serde.rs/
 * serde 是用来序列化和反序列化数据的lib，处理保护json/yaml/xml等格式
 * 
 * 
 * [features] - https://course.rs/cargo/reference/features/intro.html
 * 条件编译. 如下是放弃default选项而给指定选项，运行起来没有
 * cargo run --no-default-features --features="cli"
 * 
 * run: cargo run random
 *
 * [dependencies] 有如下的方式：
 * serde = { version = "1.0.118", features = ["derive"] } -- 注意feature
 * crossterm = { version = "0.23", optional = true } -- 注意optional
 * lazy_static = "1.4.0"
 * rand = "*" # * -> the latest
 *
 * features可以在代码中获取 - 进而配置。如下的意思：如果定义了webp这个feature那么就引入webp这个模块 
 * #[cfg(feature = "webp")]
 * pub mod webp;
 */