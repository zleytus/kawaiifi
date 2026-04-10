# Set up the project
project('kawaiifi', 'rust', version: '0.1.0', meson_version: '>= 1.4')

# Import GNOME module
gnome = import('gnome')

# Set IDs
base_id = 'fi.kawaii.kawaiifi'
application_id = base_id

# Set a couple of useful variables
bindir = get_option('prefix') / get_option('bindir')
localedir = get_option('prefix') / get_option('localedir')
datadir = get_option('prefix') / get_option('datadir')
pkgdatadir = datadir / meson.project_name()

# Enter these subdirectories and execute the meson.build files in them
subdir('data')
subdir('src')

# Execute these tasks after installing
gnome.post_install(
    gtk_update_icon_cache: true,
    glib_compile_schemas: true,
    update_desktop_database: true,
)
