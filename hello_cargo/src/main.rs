use std::io;
use colored::*; // Подключаем крейт colored

fn main() {
    // Включить поддержку ANSI для Windows 10+
    #[cfg(windows)]
    colored::control::set_virtual_terminal(true).unwrap();

    // Выводим зелёный текст
    println!("{}", "Hello, from RUST!".green());

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