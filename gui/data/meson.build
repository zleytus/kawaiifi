subdir('resources')

# Linux-specific: desktop file, GSettings schema, and icons
if host_machine.system() == 'linux'
    subdir('icons')

    desktop_conf = configuration_data()
    desktop_conf.set('APP_ID', application_id)
    desktop_conf.set('BINDIR', bindir)
    configure_file(
        input: '@0@.desktop.in'.format(base_id),
        output: '@0@.desktop'.format(application_id),
        configuration: desktop_conf,
        install: true,
        install_dir: datadir / 'applications',
    )

    metainfo_conf = configuration_data()
    metainfo_conf.set('APP_ID', application_id)
    configure_file(
        input: '@0@.metainfo.xml.in'.format(base_id),
        output: '@0@.metainfo.xml'.format(application_id),
        configuration: metainfo_conf,
        install: true,
        install_dir: datadir / 'metainfo',
    )

    install_data(
        '@0@.gschema.xml'.format(base_id),
        install_dir: datadir / 'glib-2.0' / 'schemas',
    )

    gnome.compile_schemas()

    service_conf = configuration_data()
    service_conf.set('APP_ID', application_id)
    service_conf.set('BINDIR', bindir)
    configure_file(
        input: '@0@.service.in'.format(base_id),
        output: '@0@.service'.format(application_id),
        configuration: service_conf,
        install: true,
        install_dir: datadir / 'dbus-1' / 'services',
    )
endif
