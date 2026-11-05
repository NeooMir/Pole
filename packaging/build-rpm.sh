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

# Copy spec file
cp "$SCRIPT_DIR/pole.spec" "$RPM_TOPDIR/SPECS/pole.spec"

echo "==> Running rpmbuild..."
rpmbuild --define "_topdir $RPM_TOPDIR" \
         --define "_project_root $ROOT_DIR" \
         --buildroot "$BUILD_ROOT" \
         --nodeps \
         -bb "$RPM_TOPDIR/SPECS/pole.spec"

RPM_FILE=$(find "$RPM_TOPDIR/RPMS" -name "*.rpm" | head -n 1)
cp "$RPM_FILE" "$DIST_DIR/"

echo "==> RPM package created successfully:"
ls -lh "$DIST_DIR/$(basename "$RPM_FILE")"
