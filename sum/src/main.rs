use std::io;

fn main() {
    println!("Введите первое число i8:");
    let num1: i8 = read_number();

    println!("Введите второе число i8:");
    let num2: i8 = read_number();

    match num1.checked_add(num2) {
        Some(sum) => println!("Сумма чисел: {}", sum),
        None => println!("Ошибка: переполнение при сложении!"),
    }

    // Ожидание нажатия Enter перед завершением
    wait_for_enter();
}

fn read_number() -> i8 {
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Не удалось прочитать строку");

        match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => println!("Пожалуйста, введите корректное число:"),
        };
    }
}

fn wait_for_enter() {
    println!("\nНажмите Enter для выхода...");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Не удалось прочитать строку");
}