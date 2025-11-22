// Hello World with arguments
fn main(args) {
    let args: Vec<String> = std::env::args().collect();
    println!("Hello, world!");
}
