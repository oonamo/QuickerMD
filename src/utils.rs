pub fn exit(message: &str, code: i32) -> ! {
    eprintln!("{}", message);
    std::process::exit(code);
}
