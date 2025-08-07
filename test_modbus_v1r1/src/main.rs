use tokio_modbus::prelude::*;
use tokio_serial::{SerialStream};
use std::io::{self, Write};
use colored::*;
use serialport;

#[cfg(windows)]
use winapi::um::consoleapi::{GetConsoleMode, SetConsoleMode};
#[cfg(windows)]
use winapi::um::processenv::GetStdHandle;
#[cfg(windows)]
use winapi::um::winbase::{STD_OUTPUT_HANDLE, STD_ERROR_HANDLE};
#[cfg(windows)]
const ENABLE_VIRTUAL_TERMINAL_PROCESSING: u32 = 0x0004;

/// Доступные скорости передачи данных для RS-485 (в бодах)
const AVAILABLE_BAUD_RATES: (u32, u32, u32, u32, u32, u32, u32) = (
    2400, 4800, 9600, 19200, 38400, 57600, 115200
);

/// Включение поддержки цветного вывода в Windows
#[cfg(windows)]
fn enable_ansi_support() {
    unsafe {
        let stdout = GetStdHandle(STD_OUTPUT_HANDLE);
        let stderr = GetStdHandle(STD_ERROR_HANDLE);
        
        let mut mode: u32 = 0;
        if GetConsoleMode(stdout, &mut mode) != 0 {
            SetConsoleMode(stdout, mode | ENABLE_VIRTUAL_TERMINAL_PROCESSING);
        }
        
        let mut mode: u32 = 0;
        if GetConsoleMode(stderr, &mut mode) != 0 {
            SetConsoleMode(stderr, mode | ENABLE_VIRTUAL_TERMINAL_PROCESSING);
        }
    }
}

/// Заглушка для не-Windows систем
#[cfg(not(windows))]
fn enable_ansi_support() {
    // На Unix-системах цветной вывод работает по умолчанию
}

/// Функция сканирования доступных COM-портов
fn scan_available_ports(available_ports: &mut [u8; 10]) -> usize {
    let mut count = 0;
    
    println!("Сканирование доступных COM-портов...");
    
    match serialport::available_ports() {
        Ok(ports) => {
            for port in ports {
                if let Some(port_name) = port.port_name.strip_prefix("COM") {
                    if let Ok(port_num) = port_name.parse::<u8>() {
                        if count < 10 {
                            available_ports[count] = port_num;
                            println!("  Найден: COM{}", port_num);
                            count += 1;
                        }
                    }
                }
            }
            
            if count == 0 {
                println!("{}", "  COM-порты не найдены".yellow());
            } else {
                println!("{}", format!("  Всего найдено портов: {}", count).cyan());
            }
        }
        Err(e) => {
            eprintln!("{}", format!("Ошибка сканирования портов: {:?}", e).red());
        }
    }
    
    count
}

/// Функция обработки отсутствия портов
fn handle_no_ports() -> io::Result<bool> {
    println!("{}", "Доступные COM-порты не найдены!".red());
    println!("\n{}", "Выберите действие:".yellow());
    println!("  {} - выйти", "0".red());
    println!("  {} - повторить поиск", "1".green());
    
    loop {
        print!("\nВаш выбор (0-1): ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        match input.trim().parse::<u8>() {
            Ok(0) => {
                println!("{}", "Выход из программы...".yellow());
                return Ok(false); // false = выйти
            }
            Ok(1) => {
                println!("{}", "Повторяем поиск портов...".cyan());
                return Ok(true); // true = повторить поиск
            }
            _ => {
                println!("{}", "Неверный выбор! Введите 0 для выхода или 1 для повторного поиска.".red());
            }
        }
    }
}

/// Функция выбора COM-порта пользователем
fn select_com_port(available_ports: &[u8; 10], ports_count: usize) -> io::Result<Option<String>> {
    if ports_count == 0 {
        return Ok(None);
    }
    
    println!("\n{}", "Выберите COM-порт для подключения:".cyan());
    
    // Показываем список доступных портов
    for i in 0..ports_count {
        println!("  {}. COM{}", i + 1, available_ports[i]);
    }
    
    loop {
        print!("\nВведите номер порта (1-{}): ", ports_count);
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        match input.trim().parse::<usize>() {
            Ok(choice) if choice >= 1 && choice <= ports_count => {
                let selected_port = format!("COM{}", available_ports[choice - 1]);
                println!("{}", format!("Выбран порт: {}", selected_port).green());
                return Ok(Some(selected_port));
            }
            _ => {
                println!("{}", format!("Неверный выбор! Введите число от 1 до {}", ports_count).red());
            }
        }
    }
}

/// Функция выбора адреса устройства
fn select_device_address() -> io::Result<u8> {
    println!("\n{}", "Выбор адреса устройства Modbus".cyan());
    println!("Допустимый диапазон: 1-240");
    
    loop {
        print!("\nВведите адрес устройства (1-240): ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        match input.trim().parse::<u8>() {
            Ok(address) if address >= 1 && address <= 240 => {
                println!("{}", format!("Выбран адрес устройства: {}", address).green());
                return Ok(address);
            }
            Ok(address) => {
                println!("{}", format!("Недопустимый адрес: {}! Введите значение от 1 до 240.", address).red());
            }
            Err(_) => {
                println!("{}", "Неверный формат! Введите число от 1 до 240.".red());
            }
        }
    }
}

/// Функция выбора скорости передачи данных
fn select_baud_rate() -> io::Result<u32> {
    println!("\n{}", "Выбор скорости передачи данных RS-485".cyan());
    println!("Доступные скорости:");
    
    // Показываем список доступных скоростей
    println!("  1. {} бод", AVAILABLE_BAUD_RATES.0);
    println!("  2. {} бод", AVAILABLE_BAUD_RATES.1);
    println!("  3. {} бод", AVAILABLE_BAUD_RATES.2);
    println!("  4. {} бод", AVAILABLE_BAUD_RATES.3);
    println!("  5. {} бод", AVAILABLE_BAUD_RATES.4);
    println!("  6. {} бод", AVAILABLE_BAUD_RATES.5);
    println!("  7. {} бод", AVAILABLE_BAUD_RATES.6);
    
    loop {
        print!("\nВведите номер скорости (1-7): ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        match input.trim().parse::<u8>() {
            Ok(choice) if choice >= 1 && choice <= 7 => {
                let selected_baud = match choice {
                    1 => AVAILABLE_BAUD_RATES.0,
                    2 => AVAILABLE_BAUD_RATES.1,
                    3 => AVAILABLE_BAUD_RATES.2,
                    4 => AVAILABLE_BAUD_RATES.3,
                    5 => AVAILABLE_BAUD_RATES.4,
                    6 => AVAILABLE_BAUD_RATES.5,
                    7 => AVAILABLE_BAUD_RATES.6,
                    _ => unreachable!(), // Этого никогда не произойдет из-за проверки выше
                };
                println!("{}", format!("Выбрана скорость: {} бод", selected_baud).green());
                return Ok(selected_baud);
            }
            Ok(choice) => {
                println!("{}", format!("Недопустимый выбор: {}! Введите число от 1 до 7.", choice).red());
            }
            Err(_) => {
                println!("{}", "Неверный формат! Введите число от 1 до 7.".red());
            }
        }
    }
}

/// Функция ожидания нажатия Enter для завершения программы
fn wait_for_enter() -> io::Result<()> {
    println!("\nНажмите Enter для завершения программы...");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(())
}

#[tokio::main]
async fn main() -> io::Result<()> {
    // Включение поддержки цветного вывода в Windows
    enable_ansi_support();

    let mut available_ports: [u8; 10] = [0; 10]; // номера доступных ком-портов, всего не более 10

    // Цикл поиска и выбора порта
    let tty_path = loop {
        // Сканирование доступных COM-портов
        let ports_count = scan_available_ports(&mut available_ports);
        
        // Выбор COM-порта пользователем
        match select_com_port(&available_ports, ports_count)? {
            Some(port) => break port, // Порт выбран, выходим из цикла
            None => {
                // Портов нет, спрашиваем пользователя что делать
                if !handle_no_ports()? {
                    // Пользователь выбрал выход
                    wait_for_enter()?;
                    return Ok(());
                }
                // Пользователь выбрал повторить поиск, продолжаем цикл
                println!(); // Пустая строка для разделения
            }
        }
    };
    
    // Выбор адреса устройства
    let device_address = select_device_address()?;
    
    // Выбор скорости передачи данных
    let baud_rate = select_baud_rate()?;
    
    // Настройка параметров последовательного порта
    let builder = tokio_serial::new(&tty_path, baud_rate)
        .data_bits(tokio_serial::DataBits::Eight)
        .parity(tokio_serial::Parity::None)
        .stop_bits(tokio_serial::StopBits::One);

    // Открытие последовательного порта
    let port = match SerialStream::open(&builder) {
        Ok(port) => {
            println!("Последовательный порт {} успешно открыт", tty_path);
            port
        }
        Err(e) => {
            eprintln!("{}", format!("Ошибка открытия последовательного порта {}: {:?}", tty_path, e).red());
            wait_for_enter()?;
            return Ok(());
        }
    };

    // Создание контекста Modbus RTU
    let mut ctx = match rtu::connect(port).await {
        Ok(ctx) => {
            println!("Modbus RTU контекст успешно создан");
            ctx
        }
        Err(e) => {
            eprintln!("{}", format!("Ошибка создания Modbus RTU контекста: {:?}", e).red());
            wait_for_enter()?;
            return Ok(());
        }
    };

    // Параметры запроса
    let slave_addr = Slave(device_address); // Адрес устройства
    let register_addr = 21; // Номер регистра
    let quantity = 1; // Количество регистров для чтения

    // Установка адреса устройства
    ctx.set_slave(slave_addr);

    // Чтение входных регистров (функция 04)
    match ctx.read_input_registers(register_addr, quantity).await {
        Ok(data) => {
            println!("{}", format!("Прочитанное значение регистра {}: {}", register_addr, data[0]).green());
        }
        Err(e) => {
            eprintln!("{}", format!("Ошибка чтения регистра: {:?}", e).red());
            // Ожидание нажатия Enter в случае ошибки
            wait_for_enter()?;
            return Ok(());
        }
    }

    // Ожидание нажатия Enter для завершения программы
    wait_for_enter()?;

    Ok(())
}