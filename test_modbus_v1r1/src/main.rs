use tokio_modbus::prelude::*;
use tokio_serial::{SerialStream};
use std::io::{self, Write};
use std::time::Duration;
use std::fs;
use colored::*;
use serialport;
use serde::{Deserialize, Serialize};

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

/// Доступные варианты четности для RS-485
const PARITY_OPTIONS: (&str, &str, &str) = ("None", "Even", "Odd");

/// Доступные варианты стоп-битов для RS-485
const STOP_BITS_OPTIONS: (&str, &str) = ("1 стоп-бит", "2 стоп-бита");

/// Структура для хранения настроек подключения
#[derive(Serialize, Deserialize, Debug, Clone)]
struct ConnectionSettings {
    port: String,
    device_address: u8,
    baud_rate: u32,
    parity: String,
    stop_bits: u8,
}

/// Структура для метаданных
#[derive(Serialize, Deserialize, Debug)]
struct Metadata {
    last_updated: String,
    version: String,
    description: String,
}

/// Основная структура конфигурации
#[derive(Serialize, Deserialize, Debug)]
struct Config {
    connection: ConnectionSettings,
    metadata: Metadata,
}

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
    
    // Показываем список доступных скоростей в обратном порядке (от большей к меньшей)
    println!("  1. {} бод", AVAILABLE_BAUD_RATES.6);
    println!("  2. {} бод", AVAILABLE_BAUD_RATES.5);
    println!("  3. {} бод", AVAILABLE_BAUD_RATES.4);
    println!("  4. {} бод", AVAILABLE_BAUD_RATES.3);
    println!("  5. {} бод", AVAILABLE_BAUD_RATES.2);
    println!("  6. {} бод", AVAILABLE_BAUD_RATES.1);
    println!("  7. {} бод", AVAILABLE_BAUD_RATES.0);
    
    loop {
        print!("\nВведите номер скорости (1-7): ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        match input.trim().parse::<u8>() {
            Ok(choice) if choice >= 1 && choice <= 7 => {
                let selected_baud = match choice {
                    1 => AVAILABLE_BAUD_RATES.6,
                    2 => AVAILABLE_BAUD_RATES.5,
                    3 => AVAILABLE_BAUD_RATES.4,
                    4 => AVAILABLE_BAUD_RATES.3,
                    5 => AVAILABLE_BAUD_RATES.2,
                    6 => AVAILABLE_BAUD_RATES.1,
                    7 => AVAILABLE_BAUD_RATES.0,
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

/// Функция выбора четности
fn select_parity() -> io::Result<tokio_serial::Parity> {
    println!("\n{}", "Выбор четности для RS-485".cyan());
    println!("Доступные варианты четности:");
    
    // Показываем список доступных вариантов четности
    println!("  1. {} - без контроля четности", PARITY_OPTIONS.0);
    println!("  2. {} - четная четность", PARITY_OPTIONS.1);
    println!("  3. {} - нечетная четность", PARITY_OPTIONS.2);
    
    loop {
        print!("\nВведите номер четности (1-3): ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        match input.trim().parse::<u8>() {
            Ok(choice) if choice >= 1 && choice <= 3 => {
                let (selected_parity, parity_name) = match choice {
                    1 => (tokio_serial::Parity::None, PARITY_OPTIONS.0),
                    2 => (tokio_serial::Parity::Even, PARITY_OPTIONS.1),
                    3 => (tokio_serial::Parity::Odd, PARITY_OPTIONS.2),
                    _ => unreachable!(), // Этого никогда не произойдет из-за проверки выше
                };
                println!("{}", format!("Выбрана четность: {}", parity_name).green());
                return Ok(selected_parity);
            }
            Ok(choice) => {
                println!("{}", format!("Недопустимый выбор: {}! Введите число от 1 до 3.", choice).red());
            }
            Err(_) => {
                println!("{}", "Неверный формат! Введите число от 1 до 3.".red());
            }
        }
    }
}

/// Функция выбора количества стоп-битов
fn select_stop_bits() -> io::Result<tokio_serial::StopBits> {
    println!("\n{}", "Выбор количества стоп-битов для RS-485".cyan());
    println!("Доступные варианты:");
    
    // Показываем список доступных вариантов стоп-битов
    println!("  1. {} (стандартная настройка)", STOP_BITS_OPTIONS.0);
    println!("  2. {} (повышенная надежность)", STOP_BITS_OPTIONS.1);
    
    loop {
        print!("\nВведите номер стоп-битов (1-2): ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        match input.trim().parse::<u8>() {
            Ok(choice) if choice >= 1 && choice <= 2 => {
                let (selected_stop_bits, stop_bits_name) = match choice {
                    1 => (tokio_serial::StopBits::One, STOP_BITS_OPTIONS.0),
                    2 => (tokio_serial::StopBits::Two, STOP_BITS_OPTIONS.1),
                    _ => unreachable!(), // Этого никогда не произойдет из-за проверки выше
                };
                println!("{}", format!("Выбрано: {}", stop_bits_name).green());
                return Ok(selected_stop_bits);
            }
            Ok(choice) => {
                println!("{}", format!("Недопустимый выбор: {}! Введите число от 1 до 2.", choice).red());
            }
            Err(_) => {
                println!("{}", "Неверный формат! Введите число от 1 до 2.".red());
            }
        }
    }
}

/// Функция получения пути к файлу настроек
fn get_settings_path() -> String {
    // В режиме разработки (cargo run) - в корне проекта
    // В режиме release (exe файл) - рядом с exe файлом
    if cfg!(debug_assertions) {
        // Режим разработки - файл в корне проекта
        "connect_settings.json".to_string()
    } else {
        // Режим release - файл рядом с exe
        match std::env::current_exe() {
            Ok(exe_path) => {
                if let Some(exe_dir) = exe_path.parent() {
                    exe_dir.join("connect_settings.json").to_string_lossy().to_string()
                } else {
                    "connect_settings.json".to_string()
                }
            }
            Err(_) => "connect_settings.json".to_string()
        }
    }
}

/// Функция загрузки настроек из JSON файла
fn load_settings() -> io::Result<Config> {
    let settings_path = get_settings_path();
    let file_content = fs::read_to_string(&settings_path)?;
    let config: Config = serde_json::from_str(&file_content)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    Ok(config)
}

/// Функция сохранения настроек в JSON файл
fn save_settings(connection: ConnectionSettings) -> io::Result<()> {
    let metadata = Metadata {
        last_updated: chrono::Utc::now().to_rfc3339(),
        version: "1.0".to_string(),
        description: "Настройки подключения для Modbus RTU через RS-485".to_string(),
    };
    
    let config = Config {
        connection,
        metadata,
    };
    
    let json_content = serde_json::to_string_pretty(&config)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    
    let settings_path = get_settings_path();
    fs::write(&settings_path, json_content)?;
    Ok(())
}

/// Функция отображения настроек связи
fn show_connection_settings() -> io::Result<()> {
    clear_screen();
    println!("{}", "=== Текущие настройки связи ===".cyan().bold());
    
    match load_settings() {
        Ok(config) => {
            let conn = &config.connection;
            println!("\n{}", "Параметры подключения:".yellow());
            println!("  {} {}", "COM-порт:".green(), conn.port.bright_white());
            println!("  {} {}", "Адрес устройства:".green(), conn.device_address.to_string().bright_white());
            println!("  {} {} бод", "Скорость:".green(), conn.baud_rate.to_string().bright_white());
            println!("  {} {}", "Четность:".green(), conn.parity.bright_white());
            
            let stop_bits_text = match conn.stop_bits {
                1 => "1 стоп-бит",
                2 => "2 стоп-бита",
                _ => "неизвестно",
            };
            println!("  {} {}", "Стоп-биты:".green(), stop_bits_text.bright_white());
            
            println!("\n{}", "Информация о файле:".yellow());
            println!("  {} {}", "Версия:".blue(), config.metadata.version.bright_white());
            println!("  {} {}", "Обновлен:".blue(), config.metadata.last_updated.bright_white());
        }
        Err(e) => {
            eprintln!("{}", format!("Ошибка загрузки настроек: {}", e).red());
            println!("{}", "Будут использованы настройки по умолчанию.".yellow());
        }
    }
    
    Ok(())
}

/// Функция очистки экрана консоли
fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}

/// Функция ожидания нажатия Enter для продолжения
fn wait_for_continue() -> io::Result<()> {
    println!("\n{}", "Нажмите Enter для продолжения...".bright_black());
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(())
}

/// Функция изменения настроек связи
fn change_connection_settings() -> io::Result<()> {
    clear_screen();
    println!("{}", "=== Изменение настроек связи ===".cyan().bold());
    println!();
    
    // Сканирование доступных портов
    let mut available_ports: [u8; 10] = [0; 10];
    let ports_count = scan_available_ports(&mut available_ports);
    
    // Выбор COM-порта
    let port = loop {
        match select_com_port(&available_ports, ports_count)? {
            Some(port) => break port,
            None => {
                if !handle_no_ports()? {
                    return Ok(()); // Пользователь выбрал выход
                }
                println!(); // Пустая строка для разделения
                let ports_count = scan_available_ports(&mut available_ports);
                if ports_count == 0 {
                    continue;
                }
            }
        }
    };
    
    // Выбор адреса устройства
    let device_address = select_device_address()?;
    
    // Выбор скорости передачи данных
    let baud_rate = select_baud_rate()?;
    
    // Выбор четности
    let parity_enum = select_parity()?;
    let parity = match parity_enum {
        tokio_serial::Parity::None => "None".to_string(),
        tokio_serial::Parity::Even => "Even".to_string(),
        tokio_serial::Parity::Odd => "Odd".to_string(),
    };
    
    // Выбор количества стоп-битов
    let stop_bits_enum = select_stop_bits()?;
    let stop_bits = match stop_bits_enum {
        tokio_serial::StopBits::One => 1,
        tokio_serial::StopBits::Two => 2,
    };
    
    // Создание структуры настроек
    let connection_settings = ConnectionSettings {
        port,
        device_address,
        baud_rate,
        parity,
        stop_bits,
    };
    
    // Сохранение настроек в файл
    match save_settings(connection_settings) {
        Ok(()) => {
            println!("\n{}", "Настройки успешно сохранены!".green().bold());
        }
        Err(e) => {
            eprintln!("{}", format!("Ошибка сохранения настроек: {}", e).red());
        }
    }
    
    Ok(())
}

/// Функция запуска опроса с использованием сохраненных настроек
async fn start_polling() -> io::Result<()> {
    clear_screen();
    println!("{}", "=== Запуск опроса устройства ===".cyan().bold());
    
    // Загрузка настроек из файла
    let config = match load_settings() {
        Ok(config) => {
            println!("{}", "Настройки успешно загружены из файла".green());
            config
        }
        Err(e) => {
            eprintln!("{}", format!("Ошибка загрузки настроек: {}", e).red());
            println!("{}", "Убедитесь, что настройки сохранены (пункт 2 в главном меню)".yellow());
            return Err(e);
        }
    };
    
    let conn = &config.connection;
    println!("Используемые настройки:");
    println!("  COM-порт: {}", conn.port.bright_white());
    println!("  Адрес устройства: {}", conn.device_address.to_string().bright_white());
    println!("  Скорость: {} бод", conn.baud_rate.to_string().bright_white());
    println!("  Четность: {}", conn.parity.bright_white());
    
    let stop_bits_text = match conn.stop_bits {
        1 => "1 стоп-бит",
        2 => "2 стоп-бита",
        _ => "неизвестно",
    };
    println!("  Стоп-биты: {}", stop_bits_text.bright_white());
    println!();
    
    // Преобразование настроек для tokio_serial
    let parity = match conn.parity.as_str() {
        "None" => tokio_serial::Parity::None,
        "Even" => tokio_serial::Parity::Even,
        "Odd" => tokio_serial::Parity::Odd,
        _ => tokio_serial::Parity::None,
    };
    
    let stop_bits = match conn.stop_bits {
        1 => tokio_serial::StopBits::One,
        2 => tokio_serial::StopBits::Two,
        _ => tokio_serial::StopBits::One,
    };
    
    // Настройка параметров последовательного порта
    let builder = tokio_serial::new(&conn.port, conn.baud_rate)
        .data_bits(tokio_serial::DataBits::Eight)
        .parity(parity)
        .stop_bits(stop_bits);
    
    // Открытие последовательного порта
    let port = match SerialStream::open(&builder) {
        Ok(port) => {
            println!("{}", format!("Последовательный порт {} успешно открыт", conn.port).green());
            port
        }
        Err(e) => {
            eprintln!("{}", format!("Ошибка открытия последовательного порта {}: {:?}", conn.port, e).red());
            return Err(e.into());
        }
    };
    
    // Создание контекста Modbus RTU
    let mut ctx = match rtu::connect(port).await {
        Ok(ctx) => {
            println!("{}", "Modbus RTU контекст успешно создан".green());
            ctx
        }
        Err(e) => {
            eprintln!("{}", format!("Ошибка создания Modbus RTU контекста: {:?}", e).red());
            return Err(e.into());
        }
    };
    
    // Параметры запроса
    let slave_addr = Slave(conn.device_address);
    let register_addr = 21; // Номер регистра
    let quantity = 1; // Количество регистров для чтения
    
    // Установка адреса устройства
    ctx.set_slave(slave_addr);
    
    // Циклический опрос устройства каждую секунду
    println!("{}", "Начинается циклический опрос устройства (каждую секунду)...".cyan());
    println!("{}", "Нажмите Ctrl+C для остановки опроса".yellow());
    println!();
    
    let timeout_duration = Duration::from_millis(1000);
    let mut error_count = 0;
    
    loop {
        // Показываем только время
        let timestamp = chrono::Local::now().format("%H:%M:%S");
        print!("{} ", timestamp.to_string().bright_black());
        
        // Чтение входных регистров (функция 04) с таймаутом 1000 мс
        match tokio::time::timeout(timeout_duration, ctx.read_input_registers(register_addr, quantity)).await {
            Ok(Ok(data)) => {
                // Успешный ответ - сбрасываем счётчик ошибок
                error_count = 0;
                println!("{}", format!("Регистр {}: {}", register_addr, data[0]).green());
            }
            Ok(Err(e)) => {
                // Ошибка Modbus - увеличиваем счётчик
                error_count += 1;
                println!("{} {}", 
                    format!("Ошибка: {:?}", e).red(),
                    format!("(error_count: {})", error_count).yellow()
                );
            }
            Err(_) => {
                // Таймаут - увеличиваем счётчик
                error_count += 1;
                println!("{} {}", 
                    "Таймаут!".red(),
                    format!("(error_count: {})", error_count).yellow()
                );
            }
        }
        
        // Ожидание 1 секунды перед следующим опросом
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}

/// Функция отображения главного меню
fn show_main_menu() -> io::Result<u8> {
    clear_screen();
    println!("{}", "=== Modbus RTU Client ===".cyan().bold());
    println!("\n{}", "Выберите действие:".yellow());
    println!("  {} - Показать настройки связи", "1".green());
    println!("  {} - Изменить настройки связи", "2".blue());
    println!("  {} - Начать опрос", "3".magenta());
    println!("  {} - Выйти", "9".red());
    
    print!("\nВаш выбор (1-3, 9): ");
    io::stdout().flush()?;
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    
    match input.trim().parse::<u8>() {
        Ok(1) | Ok(2) | Ok(3) | Ok(9) => Ok(input.trim().parse().unwrap()),
        _ => {
            println!("{}", "Неверный выбор! Используется пункт 3 по умолчанию.".yellow());
            Ok(3)
        }
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    // Включение поддержки цветного вывода в Windows
    enable_ansi_support();

    // Главный цикл программы
    loop {
        let choice = show_main_menu()?;
        
        match choice {
            1 => {
                // Показать настройки связи
                show_connection_settings()?;
                wait_for_continue()?;
                println!(); // Пустая строка для разделения
                continue; // Возвращаемся к главному меню
            }
            2 => {
                // Изменить настройки связи
                change_connection_settings()?;
                wait_for_continue()?;
                continue; // Возвращаемся к главному меню
            }
            3 => {
                // Начать опрос с использованием сохраненных настроек
                match start_polling().await {
                    Ok(()) => return Ok(()),
                    Err(e) => {
                        eprintln!("{}", format!("Ошибка при опросе: {}", e).red());
                        wait_for_continue()?;
                        continue; // Возвращаемся к главному меню
                    }
                }
            }
            9 => {
                println!("{}", "Завершение программы...".yellow());
                return Ok(()); // Завершаем программу
            }
            _ => unreachable!(), // Этого не произойдет из-за проверки в show_main_menu
        }
    }
}