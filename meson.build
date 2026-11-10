project('pole',
  version: '1.3.0',
  meson_version: '>= 0.59.0',
  license: 'MIT',
)

i18n = import('i18n')
gnome = import('gnome')
fs = import('fs')

base_id = 'io.github.NeooMir.Pole'
datadir = get_option('prefix') / get_option('datadir')
bindir = get_option('prefix') / get_option('bindir')

dependency('glib-2.0', version: '>= 2.66')
dependency('gio-2.0', version: '>= 2.66')
dependency('gtk4', version: '>= 4.10.0')
dependency('libadwaita-1', version: '>= 1.5.0')

install_data('LICENSE', install_dir: datadir / 'licenses' / 'pole')

subdir('data')
subdir('src')
if fs.is_dir('python')
  subdir('python')
endif

gnome.post_install(
  glib_compile_schemas: false,
  gtk_update_icon_cache: true,
  update_desktop_database: true,
)
