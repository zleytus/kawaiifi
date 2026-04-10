if is_devel
    icon_name = '@0@.Devel.svg'.format(base_id)
else
    icon_name = '@0@.svg'.format(base_id)
endif

install_data(
    icon_name,
    install_dir: datadir / 'icons' / 'hicolor' / 'scalable' / 'apps',
    rename: '@0@.svg'.format(application_id),
)

install_data(
    '@0@-symbolic.svg'.format(base_id),
    install_dir: datadir / 'icons' / 'hicolor' / 'symbolic' / 'apps',
    rename: '@0@-symbolic.svg'.format(application_id),
)
