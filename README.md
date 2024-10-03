# Cargo plugin for Picodata plugins

Плагин к cargo с функциями для упрощения разработки плагинов к Пикодате.

## Установка

TODO: После публикации на crates.io принцип установки будет существенно упрощен.

```bash
git clone git@git.picodata.io:picodata/plugin/cargo.git
cd cargo
cargo install --path . --bin cargo-pike --locked --force
```

## Команды

### `--help`

Для всех команд есть флаг `--help` выводящий справку по использованию.

```bash
cargo pike --help
```

### `run`

Запуск кластера пикодаты по файлу `topology.toml`.

Пример топологии:

```toml
[tiers.default]
instances = 2
replication_factor = 3
```

```bash
cargo pike run --topology topology.toml --data-dir ./tmp
```

### `plugin new`

Создание нового проекта плагина из шаблона.

```bash
cargo pike plugin new name_of_new_plugin
```

Автоматически инициализирует в проект git. Для отключения этого поведения можно воспользоваться флагом `--without-git`.

### `plugin init`

Создание нового проекта плагина из шаблона в текущей папке.

```bash
cargo pike plugin init
```

Автоматически инициализирует в проект git. Для отключения этого поведения можно воспользоваться флагом `--without-git`.

### `plugin pack`

Сборка всех нужных для работы плагина файлов в один архив (для деплоя или поставки). Для работы требует чтобы предварительно была выполнена сборка проекта релизной версии с помощью `cargo`.

Сборка проекта

```bash
cargo build --release
```

Упаковка проекта

```bash
cargo pike plugin pack
```

Команда `plugin pack` создаст новый архив в директории `target` проекта.
