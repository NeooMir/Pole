%{!?_project_root: %define _project_root %{_builddir}}

Name:           pole
Version:        1.3.0
Release:        1%{?dist}
Summary:        Pole educational programming environment (KuMir / Robot / Turtle)
License:        MIT
URL:            https://github.com/NeooMir/Pole

# Disable debuginfo package generation for prebuilt binary
%global debug_package %{nil}

AutoReqProv:    yes

Requires:       gtk4 >= 4.10
Requires:       libadwaita >= 1.4
Requires:       python3

%description
Pole is a modern, lightweight educational IDE compatible with KuMir algorithmic
language and Python scripting, featuring Robot and Turtle performers, written in
Rust with GTK4 and Libadwaita.

%install
mkdir -p %{buildroot}%{_bindir}
mkdir -p %{buildroot}%{_datadir}/applications
mkdir -p %{buildroot}%{_datadir}/metainfo
mkdir -p %{buildroot}%{_datadir}/icons/hicolor/scalable/apps
mkdir -p %{buildroot}%{_datadir}/icons/hicolor/symbolic/apps
mkdir -p %{buildroot}%{_datadir}/licenses/pole

install -m 755 %{_project_root}/target/release/pole %{buildroot}%{_bindir}/pole
install -m 644 %{_project_root}/data/io.github.NeooMir.Pole.desktop.in %{buildroot}%{_datadir}/applications/io.github.NeooMir.Pole.desktop
install -m 644 %{_project_root}/data/io.github.NeooMir.Pole.metainfo.xml.in %{buildroot}%{_datadir}/metainfo/io.github.NeooMir.Pole.metainfo.xml
install -m 644 %{_project_root}/data/icons/hicolor/scalable/apps/io.github.NeooMir.Pole.svg %{buildroot}%{_datadir}/icons/hicolor/scalable/apps/
install -m 644 %{_project_root}/data/icons/hicolor/symbolic/apps/io.github.NeooMir.Pole-symbolic.svg %{buildroot}%{_datadir}/icons/hicolor/symbolic/apps/
install -m 644 %{_project_root}/LICENSE %{buildroot}%{_datadir}/licenses/pole/LICENSE

%files
%{_bindir}/pole
%{_datadir}/applications/io.github.NeooMir.Pole.desktop
%{_datadir}/metainfo/io.github.NeooMir.Pole.metainfo.xml
%{_datadir}/icons/hicolor/scalable/apps/io.github.NeooMir.Pole.svg
%{_datadir}/icons/hicolor/symbolic/apps/io.github.NeooMir.Pole-symbolic.svg
%{_datadir}/licenses/pole/LICENSE

%changelog
* Thu Nov 05 2026 NeoMir Team <https://github.com/NeooMir/Pole> - 1.3.0-1
- Release version 1.3.0 with Welcome screen, Python IPC support and MIT license.
