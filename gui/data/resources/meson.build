blueprints = custom_target('blueprints',
  input: files(
    'ui/window.blp',
    'ui/interface_box.blp',
    'ui/interface_popover.blp',
    'ui/scan_info_popover.blp',
    'ui/bss_table.blp',
    'ui/bss_chart.blp',
    'ui/bss_filter.blp',
    'ui/bss_elements.blp',
    'ui/bss_capability_info.blp',
    'ui/probe_request.blp',
  ),
  output: '.',
  command: [find_program('blueprint-compiler'), 'batch-compile', '--minify', '@OUTPUT@', '@CURRENT_SOURCE_DIR@', '@INPUT@'],
)

resources = gnome.compile_resources(
    'resources',
    'resources.gresource.xml',
    gresource_bundle: true,
    source_dir: [meson.current_source_dir(), meson.current_build_dir()],
    install: true,
    install_dir: pkgdatadir,
    dependencies: blueprints,
)
