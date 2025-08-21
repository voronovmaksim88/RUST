# Tauri + Vanilla TS

This template should help get you started developing with Tauri in vanilla HTML, CSS and Typescript.

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)


Способы запуска проекта:
1. Режим разработки (рекомендуется)
   Эта команда: `npm run tauri build`
   Запустит Vite dev server на http://localhost:1420
   Скомпилирует Rust код
   Откроет десктопное приложение с hot reload
2. Альтернативный способ для разработки
# Сначала запустить фронтенд
`npm run dev`

# Затем в другом терминале запустить Tauri
`npm run tauri dev`
3. Сборка для продакшена
   `npm run tauri build`
   Создаст исполняемые файлы в src-tauri/target/release/bundle/
   Что происходит при запуске:
   Фронтенд: Vite запускает TypeScript/HTML/CSS приложение
   Бэкенд: Rust код компилируется и создает нативное окно
   Интеграция: Tauri связывает веб-интерфейс с нативным приложением
   Системные требования:
   Node.js и npm (уже установлены, судя по наличию node_modules)
   Rust и Cargo (нужны для Tauri)
   Tauri CLI (уже в devDependencies)
   Судя по структуре папок, проект уже собирался раньше (есть папки target и dist), так что все зависимости должны быть готовы.
   Попробуйте выполнить npm run tauri dev в корневой папке проекта!