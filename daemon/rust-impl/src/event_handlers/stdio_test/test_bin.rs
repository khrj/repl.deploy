use std::io::{self, BufRead};

fn main() {
    println!("repl.deploy{{}}signature");
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut line).unwrap();
    println!("repl.deploy-success")
}
