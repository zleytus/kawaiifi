project('kawaiifi', 'rust', version: '0.1.0', meson_version: '>= 1.3')

gnome = import('gnome')

base_id = 'fi.kawaii.kawaiifi'
application_id = base_id

bindir = get_option('prefix') / get_option('bindir')
localedir = get_option('prefix') / get_option('localedir')
datadir = get_option('prefix') / get_option('datadir')
pkgdatadir = datadir / meson.project_name()

subdir('data')
subdir('src')

gnome.post_install(
    gtk_update_icon_cache: true,
    glib_compile_schemas: true,
    update_desktop_database: true,
)
