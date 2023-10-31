<img src="https://img.shields.io/badge/status-going project-3E3E3E?style=float&color=3cbf50"/>

# ftime

`ftime` выводит время в консоль в режиме live.

## Создание проекта

```bash
# создадим папку и перейдем в нее
$ mkdir 1-ferris-time
$ cd 1-ferris-time

# инициализируем проект с именем `ftime` в текущей папке
$ cargo init --name ftime

# добавим необходимые зависимости
$ cargo add owo-colors chrono
```

## Вспомогательная информация

Управляющие последовательности [ANSI](https://ru.wikipedia.org/wiki/Управляющие_последовательности_ANSI) можно использовать для управления выводом информации в терминал. [Здесь](https://gist.github.com/fnky/458719343aabd01cfb17a3a4f7296797) можно посмотреть большинство последовательностей с пояснениями. например:

- `ESC[2J` - очищает экран терминала
- `ESC[?25l` - делает курсор невидимым
- `ESC[10A` - передвигает курсор вверх 10 строк

[Функция](https://doc.rust-lang.org/std/thread/fn.sleep.html) `std::thread::sleep()` блокирует выполнение процесса на указанное время, например:

```rust
// остановка на 5 секунд
std::thread::sleep(std::time::Duration::from_secs(5));
```

Текущее время можно получить с помощью крейта `chrono` - его мы рассмотрели на [степе 1.7](https://stepik.org/lesson/1103425/step/7?unit=1114466).
