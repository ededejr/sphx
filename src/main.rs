use sphx;

fn main() {
    match sphx::run() {
        Ok(value) => println!("{}", value),
        Err(e) => eprintln!("Failed running command: {}", e),
    }
}
