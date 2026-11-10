fn main() {
    let mut c = calcbench::engine::Calc::new();
    for l in std::env::args().skip(1) {
        match c.eval(&l) {
            Ok(r) => println!("{l} => {r}"),
            Err(e) => println!("{l} => ERR {e}"),
        }
    }
}
