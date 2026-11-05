#!/usr/bin/env bash

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
DIST_DIR="$ROOT_DIR/dist"

WORK_DIR="/tmp/pole-deb-build"
PACKAGE_NAME="pole"
VERSION="1.3.0"
ARCH="amd64"

echo "==> Building Pole release binary..."
cargo build --release --manifest-path "$ROOT_DIR/Cargo.toml"

echo "==> Preparing DEB file structure..."
rm -rf "$WORK_DIR"
mkdir -p "$WORK_DIR/pkg/DEBIAN"
mkdir -p "$WORK_DIR/pkg/usr/bin"
mkdir -p "$WORK_DIR/pkg/usr/share/applications"
mkdir -p "$WORK_DIR/pkg/usr/share/metainfo"
mkdir -p "$WORK_DIR/pkg/usr/share/icons/hicolor/scalable/apps"
mkdir -p "$WORK_DIR/pkg/usr/share/icons/hicolor/symbolic/apps"
mkdir -p "$WORK_DIR/pkg/usr/share/doc/pole"
mkdir -p "$DIST_DIR"

install -m 755 "$ROOT_DIR/target/release/pole" "$WORK_DIR/pkg/usr/bin/pole"
install -m 644 "$ROOT_DIR/data/io.github.NeooMir.Pole.desktop.in" "$WORK_DIR/pkg/usr/share/applications/io.github.NeooMir.Pole.desktop"
install -m 644 "$ROOT_DIR/data/io.github.NeooMir.Pole.metainfo.xml.in" "$WORK_DIR/pkg/usr/share/metainfo/io.github.NeooMir.Pole.metainfo.xml"
install -m 644 "$ROOT_DIR/data/icons/hicolor/scalable/apps/io.github.NeooMir.Pole.svg" "$WORK_DIR/pkg/usr/share/icons/hicolor/scalable/apps/"
install -m 644 "$ROOT_DIR/data/icons/hicolor/symbolic/apps/io.github.NeooMir.Pole-symbolic.svg" "$WORK_DIR/pkg/usr/share/icons/hicolor/symbolic/apps/"
install -m 644 "$ROOT_DIR/LICENSE" "$WORK_DIR/pkg/usr/share/doc/pole/copyright"

# Calculate size in KB
INSTALLED_SIZE=$(du -sk "$WORK_DIR/pkg/usr" | cut -f1)

cat << EOF > "$WORK_DIR/pkg/DEBIAN/control"
Package: ${PACKAGE_NAME}
Version: ${VERSION}
Section: education
Priority: optional
Architecture: ${ARCH}
Installed-Size: ${INSTALLED_SIZE}
Maintainer: NeoMir Team <https://github.com/NeooMir/Pole>
Depends: libc6 (>= 2.34), libgtk-4-1 (>= 4.10), libadwaita-1-0 (>= 1.5)
Homepage: https://github.com/NeooMir/Pole
Description: Educational programming environment for KuMir with Robot performer
 Pole (NeoMir) is a modern KuMir-compatible educational IDE written in Rust with GTK4 and Libadwaita.
EOF

echo "==> Packaging DEB..."
cd "$WORK_DIR"
echo "2.0" > debian-binary

tar -czf control.tar.gz -C "$WORK_DIR/pkg/DEBIAN" .

cd "$WORK_DIR/pkg"
tar -czf "$WORK_DIR/data.tar.gz" --exclude='./DEBIAN' .

DEB_FILE="$DIST_DIR/${PACKAGE_NAME}_${VERSION}_${ARCH}.deb"
ar rcs "$DEB_FILE" "$WORK_DIR/debian-binary" "$WORK_DIR/control.tar.gz" "$WORK_DIR/data.tar.gz"

echo "==> DEB package created successfully:"
ls -lh "$DEB_FILE"
