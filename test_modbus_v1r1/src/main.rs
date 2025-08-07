use tokio_modbus::prelude::*;
use tokio_serial::{SerialStream};
use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    // Настройка параметров последовательного порта
    let tty_path = "COM7";
    let builder = tokio_serial::new(tty_path, 115200)
        .data_bits(tokio_serial::DataBits::Eight)
        .parity(tokio_serial::Parity::None)
        .stop_bits(tokio_serial::StopBits::One);

    // Открытие последовательного порта
    let port = SerialStream::open(&builder)?;

    // Создание контекста Modbus RTU
    let mut ctx = rtu::connect(port).await?;

    // Параметры запроса
    let slave_addr = Slave(1); // Адрес устройства
    let register_addr = 21; // Номер регистра
    let quantity = 1; // Количество регистров для чтения

    // Установка адреса устройства
    ctx.set_slave(slave_addr);

    // Чтение входных регистров (функция 04)
    match ctx.read_input_registers(register_addr, quantity).await {
        Ok(data) => {
            println!("Прочитанное значение регистра {}: {}", register_addr, data[0]);
        }
        Err(e) => {
            eprintln!("Ошибка чтения регистра: {:?}", e);
        }
    }

    Ok(())
}