fn main() {
    if let Err(err) = shell::run() {
        eprintln!("shell: {err}");
        std::process::exit(1);
    }
}
