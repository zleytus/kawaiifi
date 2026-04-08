fn main() {
    generate_c_bindings();
    generate_csharp_bindings();
}

fn generate_c_bindings() {
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
    "#ifdef _WIN32\n#include <guiddef.h>\n#endif\ntypedef struct Interface Interface;\ntypedef struct Bss Bss;\ntypedef struct Scan Scan;\ntypedef struct Ie Ie;\ntypedef struct FieldList FieldList;\ntypedef struct InterfaceList InterfaceList;\ntypedef struct BssList BssList;"
    .to_string(),
    );

    config.cpp_compat = true;

    cbindgen::generate_with_config(crate_dir, config)
        .expect("Unable to generate bindings")
        .write_to_file("include/kawaiifi.h");
}

fn generate_csharp_bindings() {
    // Generate NativeMethods.g.cs for cross-platform functionality
    csbindgen::Builder::default()
        .input_extern_file("src/bss.rs")
        .input_extern_file("src/common.rs")
        .input_extern_file("src/field.rs")
        .input_extern_file("src/ies.rs")
        .input_extern_file("src/interface.rs")
        .input_extern_file("src/scan.rs")
        .csharp_dll_name("kawaiifi")
        .generate_csharp_file("include/NativeMethods.g.cs")
        .unwrap();

    // Generate NativeMethods.Linux.g.cs for Linux-specific functionality
    csbindgen::Builder::default()
        .input_extern_file("src/linux/bss.rs")
        .input_extern_file("src/linux/interface.rs")
        .input_extern_file("src/linux/scan.rs")
        .csharp_dll_name("kawaiifi")
        .csharp_disable_emit_dll_name(true)
        .generate_csharp_file("include/NativeMethods.Linux.g.cs")
        .unwrap();

    // Generate NativeMethods.Windows.g.cs for Windows-specific functionality
    csbindgen::Builder::default()
        .input_extern_file("src/windows/bss.rs")
        .input_extern_file("src/windows/interface.rs")
        .csharp_file_header("using GUID = global::System.Guid;")
        .csharp_dll_name("kawaiifi")
        .csharp_disable_emit_dll_name(true)
        .generate_csharp_file("include/NativeMethods.Windows.g.cs")
        .unwrap();
}
