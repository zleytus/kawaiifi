blueprints = custom_target(
    'blueprints',
    input: files(
        'ui/bss_chart.blp',
        'ui/bss_elements.blp',
        'ui/bss_filter.blp',
        'ui/bss_table.blp',
        'ui/interface_list.blp',
        'ui/interface_row.blp',
        'ui/interface_toggle.blp',
        'ui/preferences_dialog.blp',
        'ui/window.blp',
    ),
    output: '.',
    command: [
        find_program('blueprint-compiler'),
        'batch-compile',
        '@OUTPUT@',
        '@CURRENT_SOURCE_DIR@',
        '@INPUT@',
    ],
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
