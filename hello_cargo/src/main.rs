use std::io;

fn main() {
    println!("Hello, Den from RUST!");

    // Переменная (неизменяемая)
    let number = 42;
    
    // Выводим значение переменной
    println!("Значение неизменяемой переменной: {}", number); 

    // Переменная (изменяемая) типа i32
    let mut var_num = 1;
    var_num += 1;
    
    // Выводим значение переменной
    println!("Значение переменной var_num: {}", var_num); 

    // Ожидаем ввода пользователя перед закрытием
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
}