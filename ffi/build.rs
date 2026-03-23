fn main() {
    let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();

    let mut config = cbindgen::Config::default();
    config.language = cbindgen::Language::C;
    config.documentation = true;
    config.include_guard = Some("kawaiifi_h".to_string());
    config.enumeration = cbindgen::EnumConfig {
        rename_variants: cbindgen::RenameRule::ScreamingSnakeCase,
        prefix_with_name: true,
        ..Default::default()
    };
    config.parse = cbindgen::ParseConfig {
        parse_deps: true,
        include: Some(vec!["kawaiifi".to_string()]),
        ..Default::default()
    };
    config.export = cbindgen::ExportConfig {
        item_types: vec![
            cbindgen::ItemType::Functions,
            cbindgen::ItemType::Enums,
            cbindgen::ItemType::Structs,
        ],
        exclude: vec!["GUID".to_string()],
        ..Default::default()
    };
    config
        .defines
        .insert("target_os = linux".to_string(), "__linux__".to_string());
    config
        .defines
        .insert("target_os = windows".to_string(), "_WIN32".to_string());
    config.after_includes = Some(
    "#ifdef _WIN32\n#include <guiddef.h>\n#endif\ntypedef struct Interface Interface;\ntypedef struct Bss Bss;\ntypedef struct Scan Scan;\ntypedef struct Ie Ie;\ntypedef struct FieldList FieldList;\ntypedef struct InterfaceList InterfaceList;"
    .to_string(),
    );

    config.cpp_compat = true;

    cbindgen::generate_with_config(crate_dir, config)
        .expect("Unable to generate bindings")
        .write_to_file("include/kawaiifi.h");
}
