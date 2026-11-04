#!/usr/bin/env bash

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
DIST_DIR="$ROOT_DIR/dist"
TARGET="x86_64-pc-windows-gnu"

mkdir -p "$DIST_DIR"

echo "============================================="
echo " Сборка Pole для Windows (Clang / LLVM)"
echo "============================================="

# 1. Проверка наличия Clang и LLD
if ! command -v clang &>/dev/null; then
    echo "Ошибка: clang не найден в системе. Установите clang (sudo dnf install clang lld)"
    exit 1
fi

if ! command -v lld &>/dev/null && ! command -v ld.lld &>/dev/null; then
    echo "Предупреждение: lld не найден, убедитесь что lld установлен для линковки Windows"
fi

echo "==> Используемый компилятор Clang:"
clang --version | head -n 1

# 2. Проверка Rust-таргета для Windows
if ! rustup target list | grep -q "$TARGET (installed)"; then
    echo "==> Добавление rustup target $TARGET..."
    rustup target add "$TARGET" || {
        echo "Предупреждение: не удалось автоматически скачать таргет $TARGET."
        echo "Если вы собираете нативно под Windows, используйте 'cargo build --release'."
    }
fi

export CC="clang"
export CXX="clang++"
export AR="llvm-ar"

echo "==> Сборка Windows бинарника (.exe) через Cargo & Clang..."
cargo build --release --target "$TARGET" --manifest-path "$ROOT_DIR/Cargo.toml"

EXE_SRC="$ROOT_DIR/target/$TARGET/release/pole.exe"
if [ -f "$EXE_SRC" ]; then
    echo "==> Бинарник успешно скомпилирован: $EXE_SRC"
    
    # 3. Упаковка в dist/
    ZIP_FILE="$DIST_DIR/pole-windows-x86_64.zip"
    TMP_PKG="/tmp/pole-win-pkg"
    rm -rf "$TMP_PKG"
    mkdir -p "$TMP_PKG"

    cp "$EXE_SRC" "$TMP_PKG/"
    cp "$ROOT_DIR/LICENSE" "$TMP_PKG/"
    cp "$ROOT_DIR/readme.md" "$TMP_PKG/"

    # Встраивание изолированного Python (Embedded Python для Windows)
    echo "==> Встраивание Embedded Python и библиотеки pole..."
    PYTHON_EMBED_ZIP="/tmp/python-3.11.9-embed-amd64.zip"
    PYTHON_EMBED_DIR="$TMP_PKG/python"
    mkdir -p "$PYTHON_EMBED_DIR"

    if [ ! -f "$PYTHON_EMBED_ZIP" ]; then
        echo "Скачивание официального Windows Embedded Python..."
        curl -sSL "https://www.python.org/ftp/python/3.11.9/python-3.11.9-embed-amd64.zip" -o "$PYTHON_EMBED_ZIP" || true
    fi

    if [ -f "$PYTHON_EMBED_ZIP" ]; then
        unzip -q -o "$PYTHON_EMBED_ZIP" -d "$PYTHON_EMBED_DIR" || true
        sed -i "s/#import site/import site/" "$PYTHON_EMBED_DIR"/*._pth 2>/dev/null || true
    fi

    # Копируем библиотеку pole внутрь встроенного Python
    cp -r "$ROOT_DIR/python/pole" "$PYTHON_EMBED_DIR/"
    
    cd "$TMP_PKG"
    zip -r "$ZIP_FILE" .
    rm -rf "$TMP_PKG"

    echo ""
    echo "============================================="
    echo " Сборка Windows завершена успешно!"
    echo " Готовый архив: $ZIP_FILE"
    echo "============================================="
else
    echo "Бинарник $EXE_SRC не найден. Проверьте вывод компилятора."
    exit 1
fi
