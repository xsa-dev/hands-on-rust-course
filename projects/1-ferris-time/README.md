<img src="https://img.shields.io/badge/status-going project-3E3E3E?style=float&color=64d42f"/>

# ftime

`ftime` выводит время в консоль в режиме live.

## Создание проекта

```bash
# создадим папку и перейдем в нее
$ mkdir 1-ferris-time
$ cd 1-ferris-time

# создедим новый проект с именем `ftime`
$ cargo init --name ftime

# добавим необходимые зависимости
$ cargo add owo-colors chrono
```

## Вспомогательная информация

Для управления вовода информации в терминал можно использовать управляющие последовательности [ANSI](https://ru.wikipedia.org/wiki/Управляющие_последовательности_ANSI), например:

- `ESC[2J` - очищает экран терминала
- `ESC[?25l` - делает курсор невидимым
- `ESC[10A` - передвигает курсор вверх 10 строк

[Здесь](https://gist.github.com/fnky/458719343aabd01cfb17a3a4f7296797) можно посмотреть большинство последовательностей с пояснениями
