use std::env;

fn main() {
    let args = env::args().skip(1).join("+").collect::<String>();
    println!("{}", args);
}
