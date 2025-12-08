# Tauri + Vue 3 + TypeScript

Десктопное приложение для управления проектами с устройствами, построенное на Tauri.

## Что такое npm?

**npm (Node Package Manager)** - это менеджер пакетов для JavaScript. Он позволяет:
- Устанавливать библиотеки и зависимости для проекта
- Управлять версиями пакетов
- Запускать скрипты для сборки и разработки

npm устанавливается автоматически вместе с Node.js.

## Предварительные требования

Перед запуском проекта необходимо установить следующие инструменты:

### 1. Node.js и npm

**Windows:**
1. Скачайте установщик с официального сайта: https://nodejs.org/
2. Рекомендуется версия LTS (Long Term Support)
3. Запустите установщик и следуйте инструкциям
4. После установки перезагрузите терминал
5. Проверьте установку:
   ```bash
   node --version
   npm --version
   ```

**Linux (Ubuntu/Debian):**
```bash
sudo apt update
sudo apt install nodejs npm
```

**macOS:**
```bash
brew install node
```

### 2. Rust и Cargo

**Windows:**
1. Скачайте rustup с https://rustup.rs/
2. Запустите установщик
3. Следуйте инструкциям в терминале
4. Перезагрузите терминал
5. Проверьте установку:
   ```bash
   rustc --version
   cargo --version
   ```

**Linux/macOS:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 3. Установка зависимостей проекта

После установки Node.js и Rust, выполните в корневой папке проекта:
```bash
npm install
```

Эта команда установит все необходимые зависимости, указанные в package.json.

## Способы запуска проекта

### 1. Режим разработки (рекомендуется)

Запустите одной командой:
```bash
npm run tauri dev
```

Что происходит при запуске:
- Vite запустит dev server на http://localhost:1420
- Rust код скомпилируется
- Откроется десктопное приложение с hot reload
- Изменения в коде автоматически обновляются в приложении

### 2. Альтернативный способ для разработки

Запустить фронтенд и бэкенд в разных терминалах:

**Терминал 1 (фронтенд):**
```bash
npm run dev
```

**Терминал 2 (Tauri приложение):**
```bash
npm run tauri dev
```

### 3. Сборка для продакшена

Создать исполняемый файл для распространения:
```bash
npm run tauri build
```

Результат сборки будет находиться в:
- `src-tauri/target/release/bundle/msi/` - установщик MSI (Windows)
- `src-tauri/target/release/bundle/nsis/` - установщик NSIS (Windows)

## Что происходит при запуске?

- **Фронтенд**: Vite запускает Vue 3 + TypeScript приложение
- **Бэкенд**: Rust код компилируется и создает нативное окно
- **Интеграция**: Tauri связывает веб-интерфейс с нативным приложением

## Возможности приложения

- 📁 Управление проектами
- 🔌 Управление устройствами (добавление, удаление)
- 📝 Ведение лога проекта
- 🔍 Сканирование доступных COM-портов
- 💾 Автоматическое сохранение данных в JSON

## Структура проекта

```
tauri-app/
├── src/                    # Vue.js фронтенд
│   ├── components/         # Vue компоненты
│   ├── App.vue            # Главный компонент
│   └── main.ts            # Точка входа
├── src-tauri/             # Rust бэкенд
│   ├── src/               # Rust исходники
│   │   ├── main.rs        # Точка входа
│   │   ├── lib.rs         # Tauri команды
│   │   └── scan_available_ports.rs
│   └── Cargo.toml         # Rust зависимости
├── public/                # Статические файлы
│   └── test_project.json  # Данные проекта
└── package.json           # npm зависимости
```

## Решение проблем

### Ошибка: "can't find crate for `core`" при сборке (Windows)

Если при выполнении `npm run tauri build` появляется ошибка:
```
error[E0463]: can't find crate for `core`
  = note: the `x86_64-pc-windows-gnu` target may not be installed
```

**Причина**: Rust пытается использовать GNU toolchain вместо MSVC.

**Решение**:

1. Удалите проблемный target:
   ```bash
   rustup target remove x86_64-pc-windows-gnu
   ```

2. Очистите кеш сборки:
   ```bash
   cd src-tauri
   cargo clean
   cd ..
   ```

3. Попробуйте собрать заново:
   ```bash
   npm run tauri build
   ```

Если проблема не решена, файл `src-tauri/.cargo/config.toml` уже настроен для использования MSVC target.
