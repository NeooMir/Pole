#!/usr/bin/env bash

set -e

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
DIST_DIR="$PROJECT_ROOT/dist"
FLATPAK_BUILD_DIR="$PROJECT_ROOT/_flatpak_build"
FLATPAK_REPO_DIR="$PROJECT_ROOT/_flatpak_repo"
APP_ID="io.github.NeooMir.Pole"
MANIFEST="$PROJECT_ROOT/$APP_ID.json"

mkdir -p "$DIST_DIR"

echo "============================================="
echo " Building Pole Flatpak package"
echo "============================================="

# 1. Update cargo-sources.json
echo "==> Generating/updating cargo-sources.json..."
python3 "$PROJECT_ROOT/build-aux/generate-cargo-sources.py" "$PROJECT_ROOT/Cargo.lock" "$PROJECT_ROOT/cargo-sources.json"

# 2. Check for flatpak-builder
if ! command -v flatpak-builder &>/dev/null; then
    echo "Warning: flatpak-builder is not installed."
    echo "To build Flatpak, install it:"
    echo "  sudo dnf install -y flatpak-builder   # Fedora / RHEL"
    echo "  sudo apt install -y flatpak-builder   # Ubuntu / Debian"
    exit 1
fi

# 3. Check for Flathub remote and GNOME SDK
echo "==> Checking Flatpak environment..."
if ! flatpak remotes | grep -q "flathub"; then
    echo "Adding Flathub repository..."
    flatpak remote-add --user --if-not-exists flathub https://dl.flathub.org/repo/flathub.flatpakrepo
fi

echo "==> Checking for GNOME 50 Sdk / Platform..."
echo "If components are not installed, run:"
echo "  flatpak install --user -y flathub org.gnome.Platform//50 org.gnome.Sdk//50 org.freedesktop.Sdk.Extension.rust-stable//26.08"
echo ""

# 4. Build with flatpak-builder
echo "==> Building Flatpak via flatpak-builder..."
flatpak-builder \
    --user \
    --disable-rofiles-fuse \
    --force-clean \
    --repo="$FLATPAK_REPO_DIR" \
    "$FLATPAK_BUILD_DIR" \
    "$MANIFEST"

# 5. Create single-file bundle (.flatpak) in dist/
BUNDLE_FILE="$DIST_DIR/pole-1.3.0.flatpak"
echo "==> Creating standalone Flatpak bundle: $BUNDLE_FILE..."
flatpak build-bundle "$FLATPAK_REPO_DIR" "$BUNDLE_FILE" "$APP_ID"

echo ""
echo "============================================="
echo " Flatpak built successfully!"
echo " Bundle for installation on any system:"
echo "   $BUNDLE_FILE"
echo " Install:"
echo "   flatpak install --user -y $BUNDLE_FILE"
echo " Run:"
echo "   flatpak run $APP_ID"
echo "============================================="
