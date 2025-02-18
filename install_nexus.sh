#!/bin/bash

# Установка необходимых пакетов
echo "Установка зависимостей..."
sudo apt update
sudo apt install -y git curl build-essential protobuf-compiler

# Установка Rust (если не установлен)
if ! command -v rustup &> /dev/null; then
    echo "Установка Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
fi

# Установка целевой архитектуры riscv32i-unknown-none-elf
echo "Установка целевой архитектуры riscv32i-unknown-none-elf..."
rustup target add riscv32i-unknown-none-elf

# Клонирование репозитория
echo "Клонирование репозитория..."
if [ -d ~/.nexus ]; then
    echo "Папка ~/.nexus уже существует. Обновление репозитория..."
    cd ~/.nexus
    git pull origin main
else
    echo "Клонирование репозитория в ~/.nexus..."
    git clone https://github.com/Toomad23/.nexus.git ~/.nexus
fi

# Переход в папку проекта
cd ~/.nexus/network-api/clients/cli

# Установка зависимостей и сборка проекта
echo "Сборка проекта..."
cargo build --release

# Проверка успешности сборки
if [ $? -eq 0 ]; then
    echo "Сборка завершена успешно!"
else
    echo "Ошибка при сборке проекта."
    exit 1
fi
