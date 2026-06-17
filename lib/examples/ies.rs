use std::error::Error;

use kawaiifi::Scan;
use kawaiifi::ies::Field;

fn main() -> Result<(), Box<dyn Error>> {
    let interface = kawaiifi::default_interface().expect("Expected to find a wireless interface");
    let scan = interface.scan_blocking()?;
    print_scan_ies(&scan);

    Ok(())
}

fn print_scan_ies(scan: &Scan) {
    for bss in scan.bss_list() {
        for ie in bss.ies() {
            println!("{} ({}) - {}", ie.name(), ie.id, ie.summary());

            for field in ie.fields() {
                print_field(&field, 2);
            }
        }
        println!();
    }
}

fn print_field(field: &Field, indent: usize) {
    println!("{}{}: {}", " ".repeat(indent), field.title(), field.value());
    for subfield in field.subfields() {
        print_field(subfield, indent + 2);
    }
}
