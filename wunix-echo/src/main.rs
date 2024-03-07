use std::env;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let message = args.join(" ");
    println!("{}", message);
}

