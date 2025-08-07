use tokio_modbus::prelude::*;
use tokio_serial::{SerialStream};
use std::io;
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

    // Сканирование доступных COM-портов
    let _ports_count = scan_available_ports(&mut available_ports);
    
    // Настройка параметров последовательного порта
    let tty_path = "COM7";
    let builder = tokio_serial::new(tty_path, 115200)
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
    let slave_addr = Slave(1); // Адрес устройства
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