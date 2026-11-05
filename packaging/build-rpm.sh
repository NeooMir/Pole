#!/usr/bin/env bash

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
DIST_DIR="$ROOT_DIR/dist"

RPM_TOPDIR="/tmp/pole-rpm-topdir"
BUILD_ROOT="/tmp/pole-rpm-buildroot"

echo "==> Building Pole release binary..."
cargo build --release --manifest-path "$ROOT_DIR/Cargo.toml"

echo "==> Preparing package file structure..."
rm -rf "$RPM_TOPDIR" "$BUILD_ROOT"
mkdir -p "$RPM_TOPDIR"/{BUILD,RPMS,SOURCES,SPECS,SRPMS}
mkdir -p "$DIST_DIR"

# Create spec file
cat << EOF > "$RPM_TOPDIR/SPECS/pole.spec"
Name:           pole
Version:        1.3.0
Release:        1%{?dist}
Summary:        Pole educational programming environment (KuMir / Robot / Turtle)
License:        MIT
URL:            https://github.com/NeooMir/Pole

AutoReqProv:    yes

%description
Pole (NeoMir) is a modern, lightweight educational IDE compatible with KuMir algorithmic language
and Python scripting, with Robot and Turtle performers, written in Rust with GTK4 and Libadwaita.

%install
mkdir -p %{buildroot}/usr/bin
mkdir -p %{buildroot}/usr/share/applications
mkdir -p %{buildroot}/usr/share/metainfo
mkdir -p %{buildroot}/usr/share/icons/hicolor/scalable/apps
mkdir -p %{buildroot}/usr/share/icons/hicolor/symbolic/apps
mkdir -p %{buildroot}/usr/share/licenses/pole

install -m 755 "$ROOT_DIR/target/release/pole" %{buildroot}/usr/bin/pole
install -m 644 "$ROOT_DIR/data/io.github.NeooMir.Pole.desktop.in" %{buildroot}/usr/share/applications/io.github.NeooMir.Pole.desktop
install -m 644 "$ROOT_DIR/data/io.github.NeooMir.Pole.metainfo.xml.in" %{buildroot}/usr/share/metainfo/io.github.NeooMir.Pole.metainfo.xml
install -m 644 "$ROOT_DIR/data/icons/hicolor/scalable/apps/io.github.NeooMir.Pole.svg" %{buildroot}/usr/share/icons/hicolor/scalable/apps/
install -m 644 "$ROOT_DIR/data/icons/hicolor/symbolic/apps/io.github.NeooMir.Pole-symbolic.svg" %{buildroot}/usr/share/icons/hicolor/symbolic/apps/
install -m 644 "$ROOT_DIR/LICENSE" %{buildroot}/usr/share/licenses/pole/LICENSE

%files
/usr/bin/pole
/usr/share/applications/io.github.NeooMir.Pole.desktop
/usr/share/metainfo/io.github.NeooMir.Pole.metainfo.xml
/usr/share/icons/hicolor/scalable/apps/io.github.NeooMir.Pole.svg
/usr/share/icons/hicolor/symbolic/apps/io.github.NeooMir.Pole-symbolic.svg
/usr/share/licenses/pole/LICENSE

%changelog
* Wed Nov 05 2026 NeoMir Team <https://github.com/NeooMir/Pole> - 1.3.0-1
- Release version 1.3.0 with Python IPC support and MIT license.
EOF

echo "==> Running rpmbuild..."
rpmbuild --define "_topdir $RPM_TOPDIR" \
         --buildroot "$BUILD_ROOT" \
         -bb "$RPM_TOPDIR/SPECS/pole.spec"

RPM_FILE=$(find "$RPM_TOPDIR/RPMS" -name "*.rpm" | head -n 1)
cp "$RPM_FILE" "$DIST_DIR/"

echo "==> RPM package created successfully:"
ls -lh "$DIST_DIR/$(basename "$RPM_FILE")"
