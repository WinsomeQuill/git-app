# GIT APP
Временный репозиторий для работы по МДК 02.02

### Описание
Простой счётчик кликов на библиотеке GTK4.

### Как запустить?
#### Windows
- Ставим Visual Studio и в Visual Studio Installer устанавливаем C++
- Скачать и установить [MSYS2](https://www.msys2.org/)
- Запустить Shell Mingw64 или Mingw32 (зависит от разрядности вашей ОС)
- С помощью pacman установить GTK ```pacman -S mingw-w64-x86_64-gtk4``` если не компилируется, то ставим [зависимости](https://packages.msys2.org/package/mingw-w64-x86_64-gtk4)
- Ставим PKG Config ```pacman -S mingw-w64-x86_64-pkg-config``` или скачиваем бинарник с [sourceforge](https://sourceforge.net/projects/pkgconfiglite/)
- Если скачали бинарник PKG Config, то распаковываем в ```C:\pkg-config-lite-0.28-1\bin``` и в системой среде указываем переменную ```PKG_CONFIG_PATH``` со значением ```C:\pkg-config-lite-0.28-1\bin```
- Ставим gcc ```pacman -S mingw-w64-x86_64-gcc```
- В системной среде добавляем в PATH пути к mingw64 / mingw32:
  - ```C:\msys64\mingw64\include```
  - ```C:\msys64\mingw64\bin```
  - ```C:\msys64\mingw64\lib```
- Скачиваем и устанавливаем Rust с [официального сайта](https://www.rust-lang.org/tools/install)
- В консоли прописываем ```rustup toolchain install stable-gnu``` и ```rustup default stable-gnu```
- И запускаем проект ```cargo run```

### Фотокарточка
![гифка](https://i.imgur.com/ddjbgV4.gif)