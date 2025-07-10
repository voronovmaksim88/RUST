use std::io;

fn main() {
    println!("Hello, Den from RUST!");
    
    // Ожидаем ввода пользователя перед закрытием
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
}