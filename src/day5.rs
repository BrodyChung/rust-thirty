// ref: https://github.com/conradludgate/wordle

// const is immutable, static can be mutable
// const是不可变，而static是可变
// const/static变量都需要全部大写
const FINAL: &[&str] = &["cigar", "rebut", "sissy"];
const BROWSERS: &'static [&'static str] = &["firefox", "chorme"];

struct Foo(u32);
static FOO: Foo = Foo(5);
static mut FOO_MUT: Foo = Foo(3);

static mut STATIC_STR: String = String::new();
// 不能以这种方式来初始化，可以使用lazy_static - static不是动态的
// static mut STATIC_STR_FAILED: String = new_str("nothing");
lazy_static! {
    static ref LAZY_STATIC_STR: String = new_str("something");
}

fn new_str(str: &str) -> String {
    String::from(str)
}