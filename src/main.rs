fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match calcbench::cli::run(&args) {
        Ok(out) => println!("{out}"),
        Err(e) => {
            eprintln!("error: {e}");
            std::process::exit(1);
        }
    }
}
