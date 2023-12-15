extern crate chrono;
use chrono::Local;
use owo_colors::OwoColorize;

// строка с кабиком Ferris
const FERRIS: &str = r#"
    _~^~^~_
\) /  o o  \ (/
  '_   -   _'
  / '-----' \
"#;

fn main() {
    loop {
        // с помощью последовательностей ANSI
        // очистите экран терминала и сделайте курсор невидимым
        print!("\x1B[2J"); // ESC[2J
        print!("\x1B[?25l"); // ESC[?25l

        // Получаем текущее локальное время
        let current_time = Local::now();
        // Форматируем время в нужном формате
        let formatted_time = current_time.format("%H-%M-%S");
        // Выводим отформатированное время с цветом текста
        println!("{}", formatted_time.to_string().red());

        // выводим крабика желтым цветом
        println!("{}", FERRIS.yellow());

        // поднимаем курсор на 10 вверх
        print!("\x1B[10A");

        // обновляю каждую секунду
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
