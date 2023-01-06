pub mod helper {
    pub fn emit_try_help(status: i32, program_name: String) {
        println!("Try '{} --help' for more information.", program_name);
        std::process::exit(status)
    }
}
