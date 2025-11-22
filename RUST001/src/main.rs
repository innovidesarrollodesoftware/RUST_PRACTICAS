// Hello World with arguments
fn main() {
    let args: Vec<String> = std::env::args().collect();
    println!("Hello, world!", args);
}
