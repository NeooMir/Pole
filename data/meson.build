desktop_file = i18n.merge_file(
  input: 'io.github.NeooMir.Pole.desktop.in',
  output: 'io.github.NeooMir.Pole.desktop',
  type: 'desktop',
  po_dir: '../po',
  install: true,
  install_dir: datadir / 'applications'
)

desktop_utils = find_program('desktop-file-validate', required: false)
if desktop_utils.found()
  test('Validate desktop file', desktop_utils, args: [desktop_file])
endif

appstream_file = i18n.merge_file(
  input: 'io.github.NeooMir.Pole.metainfo.xml.in',
  output: 'io.github.NeooMir.Pole.metainfo.xml',
  po_dir: '../po',
  install: true,
  install_dir: datadir / 'metainfo'
)

appstreamcli = find_program('appstreamcli', required: false)
if appstreamcli.found()
  test('Validate appstream file', appstreamcli,
    args: ['validate', '--no-net', '--explain', appstream_file]
  )
endif

install_data(
  'icons/hicolor/scalable/apps/io.github.NeooMir.Pole.svg',
  install_dir: datadir / 'icons' / 'hicolor' / 'scalable' / 'apps'
)

install_data(
  'icons/hicolor/symbolic/apps/io.github.NeooMir.Pole-symbolic.svg',
  install_dir: datadir / 'icons' / 'hicolor' / 'symbolic' / 'apps'
)
