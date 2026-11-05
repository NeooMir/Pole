Name:           pole
Version:        1.3.0
Release:        1%{?dist}
Summary:        Pole educational programming environment (KuMir / Robot / Turtle)
License:        MIT
URL:            https://github.com/NeooMir/Pole

Source0:        %{url}/archive/refs/tags/v%{version}.tar.gz

BuildRequires:  clang
BuildRequires:  lld
BuildRequires:  meson
BuildRequires:  ninja-build
BuildRequires:  rust
BuildRequires:  cargo
BuildRequires:  pkgconfig(gtk4)
BuildRequires:  pkgconfig(libadwaita-1)

Requires:       gtk4 >= 4.10
Requires:       libadwaita >= 1.4
Requires:       python3

%description
Pole is a modern, lightweight educational IDE compatible with KuMir algorithmic
language and Python scripting, featuring Robot and Turtle performers, written in
Rust with GTK4 and Libadwaita.

%prep
%autosetup -n Pole-%{version}

%build
export CC=clang
export CXX=clang++
%meson
%meson_build

%install
%meson_install

%files
%license LICENSE
%{_bindir}/pole
%{_datadir}/applications/io.github.NeooMir.Pole.desktop
%{_datadir}/metainfo/io.github.NeooMir.Pole.metainfo.xml
%{_datadir}/icons/hicolor/scalable/apps/io.github.NeooMir.Pole.svg
%{_datadir}/icons/hicolor/symbolic/apps/io.github.NeooMir.Pole-symbolic.svg

%changelog
* Thu Nov 05 2026 NeoMir Team <contact@neomir.org> - 1.3.0-1
- Release version 1.3.0 with Welcome screen, Python IPC support and MIT license.
