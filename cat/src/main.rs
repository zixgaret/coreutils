use cat::help_func::helper;
use cat::help_func::helper::emit_try_help;
use std::string::ToString;

static PROGRAM_NAME: &str = env!("CARGO_PKG_NAME");
static EXIT_SUCCESS: i32 = 0;

fn main() {
    println!("ccat!");
    emit_try_help(EXIT_SUCCESS, PROGRAM_NAME.to_string());
}
