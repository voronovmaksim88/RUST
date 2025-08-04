use std::io;
use colored::*; // Подключаем крейт colored

fn main() {
    // Включить поддержку ANSI для Windows 10+
    #[cfg(windows)]
    control::set_virtual_terminal(true).unwrap(); // модуль control входит в colored

    // Выводим зелёный текст
    println!("{}", "Hello, from RUST!".green());

    // Переменная (неизменяемая)
    let not_mut_number = 42;

    // Выводим значение переменной
    println!("Значение неизменяемой переменной not_mut_number: {}", not_mut_number); 

    // Переменная (изменяемая) типа i32
    let mut mut_number = 1;
    mut_number += 1;
    
    // Выводим значение изменяемой переменной
    println!("Значение изменяемой переменной mut_number: {}", mut_number); 

    // Ожидаем ввода пользователя перед закрытием
    println!("Нажмите Enter для выхода...");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
}